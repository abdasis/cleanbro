use cleanbro_core::models::{DockerPruneResult, DockerStatus};
use cleanbro_core::{AppError, AppResult, ErrorCode};

use crate::adapters::SystemDockerCli;
use crate::services::docker_service;

/// Thin command: baca status via adapter; kegagalan docker bukan error global.
#[tauri::command]
pub async fn docker_status() -> DockerStatus {
    tauri::async_runtime::spawn_blocking(|| docker_service::status(&SystemDockerCli))
        .await
        .unwrap_or_else(|error| {
            DockerStatus::unavailable(format!("pemeriksaan docker gagal: {error}"))
        })
}

/// Thin command: validasi target dan eksekusi prune terjadi di service.
#[tauri::command(rename_all = "snake_case")]
pub async fn docker_prune(targets: Vec<String>) -> AppResult<DockerPruneResult> {
    tauri::async_runtime::spawn_blocking(move || docker_service::prune(&SystemDockerCli, &targets))
        .await
        .map_err(|error| {
            AppError::new(ErrorCode::Internal, "prune docker gagal dijalankan")
                .with_detail(error.to_string())
        })?
}
