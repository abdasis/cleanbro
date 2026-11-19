use cleanbro_core::models::{DockerContainer, DockerImage, DockerPruneResult, DockerStatus};
use cleanbro_core::{AppError, AppResult, ErrorCode};

use crate::adapters::docker::{self, DockerCli, MAX_OUTPUT_BYTES};

const DF_ARGS: &[&str] = &["system", "df", "--format", "{{json .}}"];
const PS_ARGS: &[&str] = &["ps", "-a", "--format", "{{json .}}"];
const IMAGES_ARGS: &[&str] = &["image", "ls", "--format", "{{json .}}"];
const PRUNE_CONTAINERS: &[&str] = &["container", "prune", "-f"];
const PRUNE_IMAGES: &[&str] = &["image", "prune", "-f"];

/// Baca status docker; binary absen atau daemon mati jadi `available: false`.
pub fn status(cli: &dyn DockerCli) -> DockerStatus {
    let df_raw = match cli.run(DF_ARGS) {
        Ok(output) => output,
        Err(reason) => return DockerStatus::unavailable(reason),
    };
    let containers_raw = match cli.run(PS_ARGS) {
        Ok(output) => output,
        Err(reason) => return DockerStatus::unavailable(reason),
    };
    let images_raw = match cli.run(IMAGES_ARGS) {
        Ok(output) => output,
        Err(reason) => return DockerStatus::unavailable(reason),
    };

    let containers: Vec<DockerContainer> = docker::parse_containers(&containers_raw);
    let mut images: Vec<DockerImage> = docker::parse_images(&images_raw);
    docker::mark_in_use(&mut images, &containers);

    DockerStatus {
        available: true,
        reason: None,
        df: Some(docker::parse_df(&df_raw)),
        containers,
        images,
    }
}

/// Prune hanya target yang disetujui v0.1; volume dan `image prune -a` ditolak.
pub fn prune(cli: &dyn DockerCli, targets: &[String]) -> AppResult<DockerPruneResult> {
    let targets = parse_targets(targets)?;
    if targets.is_empty() {
        return Ok(DockerPruneResult::default());
    }

    let before = df_used_bytes(cli)?;
    let mut result = DockerPruneResult::default();
    for target in targets {
        let output = cli.run(target.args()).map_err(cli_error)?;
        result.deleted += count_deleted(&output);
        append_capped(&mut result.output, &output);
    }
    let after = df_used_bytes(cli)?;
    result.freed_bytes = before.saturating_sub(after);
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Containers,
    ImagesDangling,
}

impl Target {
    fn args(self) -> &'static [&'static str] {
        match self {
            Self::Containers => PRUNE_CONTAINERS,
            Self::ImagesDangling => PRUNE_IMAGES,
        }
    }
}

fn parse_targets(targets: &[String]) -> AppResult<Vec<Target>> {
    let mut parsed = Vec::with_capacity(targets.len());
    for raw in targets {
        let target = match raw.trim() {
            "containers" => Target::Containers,
            "images_dangling" => Target::ImagesDangling,
            other => {
                return Err(AppError::new(
                    ErrorCode::InvalidPath,
                    format!("target prune docker tidak dikenal: {other}"),
                ))
            }
        };
        parsed.push(target);
    }
    Ok(parsed)
}

fn df_used_bytes(cli: &dyn DockerCli) -> AppResult<u64> {
    let output = cli.run(DF_ARGS).map_err(|reason| {
        AppError::new(ErrorCode::Io, "daemon docker tidak tersedia").with_detail(reason)
    })?;
    Ok(docker::parse_df_used_bytes(&output))
}

fn cli_error(reason: String) -> AppError {
    AppError::new(ErrorCode::Io, "perintah prune docker gagal").with_detail(reason)
}

/// Hitung objek yang benar-benar dihapus dari output prune.
fn count_deleted(output: &str) -> u64 {
    output
        .lines()
        .map(str::trim)
        .filter(|line| {
            !line.is_empty()
                && !line.starts_with("Deleted ")
                && !line.starts_with("Total reclaimed")
                && (line.starts_with("deleted:")
                    || line.starts_with("untagged:")
                    || line.chars().all(|c| c.is_ascii_hexdigit()))
        })
        .count() as u64
}

fn append_capped(output: &mut String, chunk: &str) {
    for character in chunk.chars() {
        if output.len() + character.len_utf8() > MAX_OUTPUT_BYTES {
            break;
        }
        output.push(character);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_deleted_objects_from_prune_output() {
        let containers =
            "Deleted Containers:\nabc123def456\n789abc123def\n\nTotal reclaimed space: 12MB\n";
        assert_eq!(count_deleted(containers), 2);
        let images = "Deleted Images:\nuntagged: nginx@sha256:aaa\ndeleted: sha256:bbb\n\nTotal reclaimed space: 1.2GB\n";
        assert_eq!(count_deleted(images), 2);
        assert_eq!(count_deleted(""), 0);
    }

    #[test]
    fn rejects_unknown_target() {
        let error = parse_targets(&["volumes".to_owned()]).unwrap_err();
        assert_eq!(error.code, ErrorCode::InvalidPath);
        assert!(parse_targets(&["containers".to_owned(), "images_dangling".to_owned()]).is_ok());
    }

    #[test]
    fn append_capped_stops_at_limit() {
        let mut output = String::new();
        append_capped(&mut output, &"x".repeat(MAX_OUTPUT_BYTES + 10));
        assert_eq!(output.len(), MAX_OUTPUT_BYTES);
    }
}
