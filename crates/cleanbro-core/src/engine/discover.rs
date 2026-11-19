use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use jwalk::WalkDir;

use crate::engine::cancel::CancellationToken;
use crate::rules::matcher::normalize;
use crate::rules::{Matcher, Rule};

/// Penanda root proyek untuk label UI dan prasyarat build.
pub const PROJECT_MARKERS: &[&str] = &[
    "package.json",
    "Cargo.toml",
    "composer.json",
    "build.gradle",
    "pubspec.yaml",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub rule: &'static Rule,
    pub path: PathBuf,
}

type Visit = dyn Fn(&Path) + Send + Sync;

/// Hasil fase pencarian kandidat; `partial` bila deadline tercapai.
#[derive(Debug, Default)]
pub struct Discovery {
    pub candidates: Vec<Candidate>,
    pub partial: bool,
}

/// Telusuri `$HOME` sekali, catat direktori yang cocok dengan rule terpilih,
/// lalu jangan turun ke dalam artefak tersebut. Direktori sistem, mount lain,
/// dan `.git` dipangkas.
pub fn discover(
    home: &Path,
    matcher: &Matcher,
    rules: &[&'static Rule],
    cancel: &CancellationToken,
    deadline: Duration,
    on_visit: Arc<Visit>,
) -> Discovery {
    let started = Instant::now();
    let timed_out = Arc::new(AtomicBool::new(false));
    let wanted: HashSet<&'static str> = rules.iter().map(|rule| rule.id).collect();
    let found = Arc::new(Mutex::new(Vec::new()));
    let depth = crate::rules::catalog::max_depth(rules);
    let matcher = matcher.clone();
    let cancel_cb = cancel.clone();
    let found_cb = Arc::clone(&found);
    let timed_out_cb = Arc::clone(&timed_out);
    let home_device = device_of(home);

    let walker = WalkDir::new(home)
        .max_depth(depth)
        .skip_hidden(false)
        .follow_links(false)
        .process_read_dir(move |_, _, _, children| {
            for entry in children.iter().filter_map(|entry| entry.as_ref().ok()) {
                on_visit(&entry.path());
            }
            if cancel_cb.is_cancelled() || started.elapsed() > deadline {
                timed_out_cb.store(true, Ordering::Relaxed);
                children.clear();
                return;
            }
            children.retain(|entry| {
                let Ok(entry) = entry else { return true };
                if !entry.file_type.is_dir() {
                    return true;
                }
                let path = normalize(&entry.path());
                if is_pruned(&path) {
                    return false;
                }
                if let (Some(home_device), Ok(meta)) = (home_device, entry.metadata()) {
                    if device_of_meta(&meta) != Some(home_device) {
                        return false;
                    }
                }
                if let Some(rule) = matcher.match_rule(&path) {
                    if wanted.contains(rule.id) {
                        lock(&found_cb).push(Candidate { rule, path });
                        return false;
                    }
                }
                true
            });
        });

    for entry in walker {
        if cancel.is_cancelled() || timed_out.load(Ordering::Relaxed) {
            break;
        }
        // Iterator tetap harus dikonsumsi agar worker jwalk berhenti rapi.
        let _ = entry;
    }

    let mut candidates = lock(&found).clone();
    candidates.sort_by(|a, b| (a.rule.id, &a.path).cmp(&(b.rule.id, &b.path)));
    Discovery {
        partial: timed_out.load(Ordering::Relaxed) || cancel.is_cancelled(),
        candidates,
    }
}

/// Nama proyek = direktori terdekat (di bawah `$HOME`) yang punya penanda.
pub fn project_name(path: &Path, home: &Path) -> Option<String> {
    let mut dir = path.parent()?;
    while dir.starts_with(home) && dir != home {
        if PROJECT_MARKERS
            .iter()
            .any(|marker| dir.join(marker).is_file())
        {
            return dir
                .file_name()
                .map(|name| name.to_string_lossy().into_owned());
        }
        dir = dir.parent()?;
    }
    None
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn is_pruned(path: &Path) -> bool {
    ["/proc", "/sys", "/dev", "/run"]
        .iter()
        .any(|root| path.starts_with(root))
        || path
            .components()
            .any(|component| component.as_os_str() == ".git")
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
