use std::path::Path;
use std::time::{Duration, Instant};

use jwalk::WalkDir;

use crate::engine::cancel::CancellationToken;

/// Batas default sesuai ARCHITECTURE bagian 4.
pub const MAX_FILES: u64 = 5_000_000;
pub const DEADLINE: Duration = Duration::from_secs(120);

#[derive(Debug, Clone)]
pub struct WalkOptions {
    pub max_depth: usize,
    pub max_files: u64,
    pub deadline: Duration,
}

impl Default for WalkOptions {
    fn default() -> Self {
        Self {
            max_depth: usize::MAX,
            max_files: MAX_FILES,
            deadline: DEADLINE,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WalkResult {
    pub bytes: u64,
    pub files: u64,
    pub dirs: u64,
    pub partial: bool,
    pub cancelled: bool,
}

const PRUNED_ROOTS: &[&str] = &["/proc", "/sys", "/dev", "/run"];

/// Ukur isi direktori secara paralel. `st_blocks * 512` dipakai agar sama
/// dengan `du`; `len()` jadi cadangan bila blok tidak tersedia.
pub fn walk(root: &Path, opts: &WalkOptions, cancel: &CancellationToken) -> WalkResult {
    if cancel.is_cancelled() {
        return WalkResult {
            cancelled: true,
            partial: true,
            ..WalkResult::default()
        };
    }
    let started = Instant::now();
    let root_device = device_of(root);
    let cancel_cb = cancel.clone();
    let walker = WalkDir::new(root)
        .max_depth(opts.max_depth)
        .skip_hidden(false)
        .follow_links(false)
        .process_read_dir(move |_, _, _, children| {
            if cancel_cb.is_cancelled() {
                children.clear();
                return;
            }
            children.retain(|entry| match entry {
                Ok(entry) => {
                    if !entry.file_type.is_dir() {
                        return true;
                    }
                    let path = entry.path();
                    if is_pruned_dir(&path) {
                        return false;
                    }
                    match (root_device, entry.metadata()) {
                        (Some(expected), Ok(meta)) => device_of_meta(&meta) == Some(expected),
                        _ => true,
                    }
                }
                Err(_) => true,
            });
        });

    let mut result = WalkResult::default();
    let mut visited: u64 = 0;
    for entry in walker {
        if cancel.is_cancelled() {
            result.cancelled = true;
            result.partial = true;
            break;
        }
        let Ok(entry) = entry else { continue };
        if entry.depth == 0 {
            continue;
        }
        if entry.file_type.is_dir() {
            result.dirs += 1;
        } else if entry.file_type.is_file() {
            result.files += 1;
            result.bytes += entry.metadata().map(entry_size).unwrap_or(0);
            if result.files >= opts.max_files {
                result.partial = true;
                break;
            }
        }
        visited += 1;
        if visited.is_multiple_of(4096) && started.elapsed() > opts.deadline {
            result.partial = true;
            break;
        }
    }
    result
}

fn entry_size(meta: std::fs::Metadata) -> u64 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let blocks = meta.blocks().saturating_mul(512);
        if blocks > 0 {
            return blocks;
        }
    }
    meta.len()
}

fn is_pruned_dir(path: &Path) -> bool {
    PRUNED_ROOTS.iter().any(|root| path.starts_with(root))
}

#[cfg(unix)]
fn device_of(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).ok().map(|meta| meta.dev())
}

#[cfg(not(unix))]
fn device_of(_path: &Path) -> Option<u64> {
    None
}

#[cfg(unix)]
fn device_of_meta(meta: &std::fs::Metadata) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    Some(meta.dev())
}

#[cfg(not(unix))]
fn device_of_meta(_meta: &std::fs::Metadata) -> Option<u64> {
    None
}
