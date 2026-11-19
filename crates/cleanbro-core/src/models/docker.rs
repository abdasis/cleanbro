use serde::{Deserialize, Serialize};

/// Status adapter docker untuk UI; ketiadaan docker bukan error global.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerStatus {
    pub available: bool,
    pub reason: Option<String>,
    pub df: Option<DockerDiskUsage>,
    pub containers: Vec<DockerContainer>,
    pub images: Vec<DockerImage>,
}

impl DockerStatus {
    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self {
            available: false,
            reason: Some(reason.into()),
            df: None,
            containers: Vec::new(),
            images: Vec::new(),
        }
    }
}

/// Ringkasan `docker system df` yang sudah diparse ke byte.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerDiskUsage {
    pub images_total: u64,
    pub images_active: u64,
    pub containers_total: u64,
    pub containers_active: u64,
    pub volumes_total: u64,
    pub local_volumes: u64,
    pub reclaimable_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerImage {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size_bytes: u64,
    pub in_use: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerPruneResult {
    pub freed_bytes: u64,
    pub deleted: u64,
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_has_empty_payload() {
        let status = DockerStatus::unavailable("daemon mati");
        assert!(!status.available);
        assert_eq!(status.reason.as_deref(), Some("daemon mati"));
        assert!(status.df.is_none());
        assert!(status.containers.is_empty());
        assert!(status.images.is_empty());
    }

    #[test]
    fn status_serializes_snake_case() {
        let status = DockerStatus::unavailable("tidak ada");
        let json = serde_json::to_value(&status).unwrap();
        assert_eq!(json["available"], false);
        assert_eq!(json["reason"], "tidak ada");
        assert_eq!(json["df"], serde_json::Value::Null);
    }
}
