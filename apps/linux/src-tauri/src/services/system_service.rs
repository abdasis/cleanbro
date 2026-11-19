use std::path::{Path, PathBuf};

use cleanbro_core::{AppError, AppResult, DiskSummary, ErrorCode};
use sysinfo::Disks;

/// `$HOME` pengguna; core tidak pernah menebak lokasi ini sendiri.
pub fn home_dir() -> AppResult<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "HOME tidak diketahui"))
}

/// Ringkasan filesystem tempat `$HOME` berada (FR-01).
pub fn disk_summary_for(home: &Path) -> AppResult<DiskSummary> {
    let home = home.canonicalize().unwrap_or_else(|_| home.to_path_buf());
    let disks = Disks::new_with_refreshed_list().map_err(|error| {
        AppError::new(ErrorCode::Io, "gagal membaca daftar disk").with_detail(error.to_string())
    })?;
    let disk = disks
        .iter()
        .filter(|disk| home.starts_with(disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().components().count())
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "filesystem $HOME tidak ditemukan"))?;

    Ok(DiskSummary::from_space(
        disk.total_space(),
        disk.available_space(),
        disk.mount_point().to_string_lossy(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_dir_is_absolute() {
        let home = home_dir().expect("HOME tersedia di mesin uji");
        assert!(home.is_absolute());
    }

    #[test]
    fn disk_summary_for_home_matches_fs() {
        let home = home_dir().unwrap();
        let summary = disk_summary_for(&home).expect("ringkasan disk");
        assert!(summary.total > 0);
        assert!(summary.free <= summary.total);
        assert!(summary.used <= summary.total);
        assert!(!summary.mount.is_empty());
        assert!((0.0..=100.0).contains(&summary.percent));
    }
}
