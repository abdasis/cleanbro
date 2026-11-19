use cleanbro_core::models::{DockerContainer, DockerDiskUsage, DockerImage};
use serde::Deserialize;

/// Parse ukuran docker ("512B", "1.2GB", "2.5GB (10%)") ke byte SI.
pub fn parse_size(raw: &str) -> u64 {
    let token = raw.split_whitespace().next().unwrap_or("");
    let number_end = token
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(token.len());
    let (number, unit) = token.split_at(number_end);
    let Ok(value) = number.parse::<f64>() else {
        return 0;
    };
    let multiplier = match unit.trim().to_ascii_uppercase().as_str() {
        "KB" => 1e3,
        "MB" => 1e6,
        "GB" => 1e9,
        "TB" => 1e12,
        "PB" => 1e15,
        _ => 1.0,
    };
    (value * multiplier).round() as u64
}

#[derive(Debug, Deserialize)]
struct DfRow {
    #[serde(rename = "Type", default)]
    kind: String,
    #[serde(rename = "TotalCount", default)]
    total: String,
    #[serde(rename = "Active", default)]
    active: String,
    #[serde(rename = "Size", default)]
    size: String,
    #[serde(rename = "Reclaimable", default)]
    reclaimable: String,
}

#[derive(Debug, Deserialize)]
struct PsRow {
    #[serde(rename = "ID", default)]
    id: String,
    #[serde(rename = "Names", default)]
    names: String,
    #[serde(rename = "Image", default)]
    image: String,
    #[serde(rename = "State", default)]
    state: String,
    #[serde(rename = "Size", default)]
    size: String,
}

#[derive(Debug, Deserialize)]
struct ImageRow {
    #[serde(rename = "ID", default)]
    id: String,
    #[serde(rename = "Repository", default)]
    repository: String,
    #[serde(rename = "Tag", default)]
    tag: String,
    #[serde(rename = "Size", default)]
    size: String,
}

fn rows<T: for<'de> Deserialize<'de>>(input: &str) -> Vec<T> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

/// `docker system df --format '{{json .}}'` -> ringkasan byte.
pub fn parse_df(input: &str) -> DockerDiskUsage {
    let mut usage = DockerDiskUsage::default();
    for row in rows::<DfRow>(input) {
        let total = parse_size(&row.total);
        let active = parse_size(&row.active);
        match row.kind.as_str() {
            "Images" => {
                usage.images_total = total;
                usage.images_active = active;
            }
            "Containers" => {
                usage.containers_total = total;
                usage.containers_active = active;
            }
            "Local Volumes" => {
                usage.volumes_total = total;
                usage.local_volumes = active;
            }
            _ => {}
        }
        usage.reclaimable_bytes = usage
            .reclaimable_bytes
            .saturating_add(parse_size(&row.reclaimable));
    }
    usage
}

/// Total `Size` seluruh baris df; dipakai menghitung freed bytes prune.
pub fn parse_df_used_bytes(input: &str) -> u64 {
    rows::<DfRow>(input)
        .iter()
        .map(|row| parse_size(&row.size))
        .fold(0u64, u64::saturating_add)
}

/// `docker ps -a --format '{{json .}}'` -> daftar container.
pub fn parse_containers(input: &str) -> Vec<DockerContainer> {
    rows::<PsRow>(input)
        .into_iter()
        .map(|row| DockerContainer {
            id: row.id,
            name: row.names,
            image: row.image,
            state: row.state,
            size_bytes: parse_size(&row.size),
        })
        .collect()
}

/// `docker image ls --format '{{json .}}'` -> daftar image (in_use diisi terpisah).
pub fn parse_images(input: &str) -> Vec<DockerImage> {
    rows::<ImageRow>(input)
        .into_iter()
        .map(|row| DockerImage {
            id: row.id,
            repository: row.repository,
            tag: row.tag,
            size_bytes: parse_size(&row.size),
            in_use: false,
        })
        .collect()
}

/// Tandai image yang direferensikan container hidup/mati.
pub fn mark_in_use(images: &mut [DockerImage], containers: &[DockerContainer]) {
    for image in images {
        let reference = format!("{}:{}", image.repository, image.tag);
        image.in_use = containers.iter().any(|container| {
            container.image == reference
                || container.image == image.id
                || (image.tag == "latest" && container.image == image.repository)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_si_sizes() {
        assert_eq!(parse_size("0B"), 0);
        assert_eq!(parse_size("0"), 0);
        assert_eq!(parse_size("512B"), 512);
        assert_eq!(parse_size("1.2GB"), 1_200_000_000);
        assert_eq!(parse_size("3TB"), 3_000_000_000_000);
        assert_eq!(parse_size("2.5GB (10%)"), 2_500_000_000);
        assert_eq!(parse_size(""), 0);
        assert_eq!(parse_size("N/A"), 0);
    }

    #[test]
    fn parses_df_rows() {
        let input = concat!(
            r#"{"Type":"Images","TotalCount":"3","Active":"2","Size":"24.8GB","Reclaimable":"2.5GB (10%)"}"#,
            "\n",
            r#"{"Type":"Containers","TotalCount":"4","Active":"1","Size":"12.3MB","Reclaimable":"12MB (97%)"}"#,
            "\n",
            r#"{"Type":"Local Volumes","TotalCount":"30","Active":"3","Size":"2.5GB","Reclaimable":"2.1GB (84%)"}"#,
            "\n",
            r#"{"Type":"Build Cache","TotalCount":"10","Active":"0","Size":"1.1GB","Reclaimable":"1GB (90%)"}"#,
        );
        let usage = parse_df(input);
        assert_eq!(usage.images_total, 3);
        assert_eq!(usage.images_active, 2);
        assert_eq!(usage.containers_total, 4);
        assert_eq!(usage.containers_active, 1);
        assert_eq!(usage.volumes_total, 30);
        assert_eq!(usage.local_volumes, 3);
        assert_eq!(usage.reclaimable_bytes, 5_612_000_000);
        assert_eq!(parse_df_used_bytes(input), 28_412_300_000);
    }

    #[test]
    fn parses_containers_and_marks_images() {
        let ps = concat!(
            r#"{"ID":"abc123def456","Names":"web","Image":"nginx:latest","State":"exited","Size":"1.2MB (virtual 180MB)"}"#,
            "\n",
            r#"{"ID":"def456abc123","Names":"db","Image":"postgres:16","State":"running","Size":"0B (virtual 440MB)"}"#,
        );
        let images_raw = concat!(
            r#"{"ID":"sha256:aaa","Repository":"nginx","Tag":"latest","Size":"187MB"}"#,
            "\n",
            r#"{"ID":"sha256:bbb","Repository":"<none>","Tag":"<none>","Size":"1.2GB"}"#,
        );
        let containers = parse_containers(ps);
        assert_eq!(containers.len(), 2);
        assert_eq!(containers[0].name, "web");
        assert_eq!(containers[0].size_bytes, 1_200_000);
        assert_eq!(containers[1].state, "running");

        let mut images = parse_images(images_raw);
        assert_eq!(images.len(), 2);
        assert_eq!(images[1].size_bytes, 1_200_000_000);
        mark_in_use(&mut images, &containers);
        assert!(images[0].in_use);
        assert!(!images[1].in_use);
    }
}
