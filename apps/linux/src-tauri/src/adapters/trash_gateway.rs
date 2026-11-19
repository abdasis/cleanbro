use std::path::{Path, PathBuf};

use cleanbro_core::rules::matcher::normalize;

use super::TrashGateway;

/// Implementasi nyata memakai crate `trash` (freedesktop spec di Linux).
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemTrash;

impl TrashGateway for SystemTrash {
    fn trash(&self, path: &Path) -> Result<String, String> {
        let original = normalize(path);
        trash::delete(path).map_err(|error| error.to_string())?;
        let items = trash::os_limited::list().map_err(|error| error.to_string())?;
        items
            .into_iter()
            .filter(|item| same_path(&item.original_path(), &original))
            .max_by_key(|item| item.time_deleted)
            .map(|item| item.id.to_string_lossy().into_owned())
            .ok_or_else(|| "item trash tidak ditemukan setelah dipindah".to_owned())
    }

    fn restore(&self, trash_ref: &str, target: &Path) -> Result<(), String> {
        let item = find_item(trash_ref)?;
        if !same_path(&item.original_path(), &normalize(target)) {
            return Err("path asal item trash tidak cocok".to_owned());
        }
        trash::os_limited::restore_all(std::iter::once(item)).map_err(|error| error.to_string())
    }

    fn exists(&self, trash_ref: &str) -> bool {
        find_item(trash_ref).is_ok()
    }
}

fn find_item(trash_ref: &str) -> Result<trash::TrashItem, String> {
    let items = trash::os_limited::list().map_err(|error| error.to_string())?;
    items
        .into_iter()
        .find(|item| item.id.to_string_lossy() == trash_ref)
        .ok_or_else(|| "item tidak lagi ada di trash".to_owned())
}

fn same_path(left: &Path, right: &Path) -> bool {
    let left: PathBuf = normalize(left);
    left == normalize(right)
}
