use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use cleanbro_core::ErrorCode;
use sapu_bersih::adapters::DockerCli;
use sapu_bersih::services::docker_service;

const DF_KEY: &str = "system df --format {{json .}}";
const CONT_PRUNE_KEY: &str = "container prune -f";
const IMAGE_PRUNE_KEY: &str = "image prune -f";

#[derive(Default)]
struct MockCli {
    responses: Mutex<HashMap<String, VecDeque<Result<String, String>>>>,
}

impl MockCli {
    fn new() -> Self {
        Self::default()
    }

    fn push(self, key: &str, value: Result<&str, &str>) -> Self {
        let result = value
            .map(|text| text.to_owned())
            .map_err(|text| text.to_owned());
        self.responses
            .lock()
            .unwrap()
            .entry(key.to_owned())
            .or_default()
            .push_back(result);
        self
    }
}

impl DockerCli for MockCli {
    fn run(&self, args: &[&str]) -> Result<String, String> {
        let key = args.join(" ");
        self.responses
            .lock()
            .unwrap()
            .get_mut(&key)
            .and_then(VecDeque::pop_front)
            .unwrap_or_else(|| Err(format!("tidak ada fixture untuk: {key}")))
    }
}

const DF: &str = concat!(
    r#"{"Type":"Images","TotalCount":"3","Active":"2","Size":"24.8GB","Reclaimable":"2.5GB (10%)"}"#,
    "\n",
    r#"{"Type":"Containers","TotalCount":"4","Active":"1","Size":"12.3MB","Reclaimable":"12MB (97%)"}"#,
    "\n",
    r#"{"Type":"Local Volumes","TotalCount":"30","Active":"3","Size":"2.5GB","Reclaimable":"2.1GB (84%)"}"#,
    "\n",
    r#"{"Type":"Build Cache","TotalCount":"10","Active":"0","Size":"1.1GB","Reclaimable":"1GB (90%)"}"#,
);

const PS: &str = concat!(
    r#"{"ID":"abc123def456","Names":"web","Image":"nginx:latest","State":"exited","Size":"1.2MB (virtual 180MB)"}"#,
    "\n",
    r#"{"ID":"def456abc123","Names":"db","Image":"postgres:16","State":"running","Size":"0B (virtual 440MB)"}"#,
);

const IMAGES: &str = concat!(
    r#"{"ID":"sha256:aaa","Repository":"nginx","Tag":"latest","Size":"187MB"}"#,
    "\n",
    r#"{"ID":"sha256:bbb","Repository":"<none>","Tag":"<none>","Size":"1.2GB"}"#,
);

#[test]
fn status_parses_real_cli_fixtures() {
    let cli = MockCli::new()
        .push(DF_KEY, Ok(DF))
        .push("ps -a --format {{json .}}", Ok(PS))
        .push("image ls --format {{json .}}", Ok(IMAGES));

    let status = docker_service::status(&cli);
    assert!(status.available);
    assert!(status.reason.is_none());

    let df = status.df.expect("df terisi");
    assert_eq!(df.images_total, 3);
    assert_eq!(df.images_active, 2);
    assert_eq!(df.volumes_total, 30);
    assert_eq!(df.local_volumes, 3);
    assert_eq!(df.reclaimable_bytes, 5_612_000_000);

    assert_eq!(status.containers.len(), 2);
    assert_eq!(status.containers[0].name, "web");
    assert_eq!(status.containers[0].size_bytes, 1_200_000);

    assert_eq!(status.images.len(), 2);
    assert!(status.images[0].in_use);
    assert!(!status.images[1].in_use);
    assert_eq!(status.images[1].size_bytes, 1_200_000_000);
}

#[test]
fn status_is_unavailable_when_docker_absent() {
    let cli = MockCli::new().push(
        DF_KEY,
        Err("docker tidak dapat dijalankan: No such file or directory"),
    );
    let status = docker_service::status(&cli);
    assert!(!status.available);
    assert!(status.reason.unwrap().contains("No such file"));
    assert!(status.df.is_none());
    assert!(status.containers.is_empty());
    assert!(status.images.is_empty());
}

#[test]
fn prune_rejects_unknown_target_without_running_cli() {
    let cli = MockCli::new();
    let error = docker_service::prune(&cli, &["volumes".to_owned()]).unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidPath);
    assert!(error.message.contains("volumes"));
}

#[test]
fn prune_refuses_when_daemon_unavailable() {
    let cli = MockCli::new().push(DF_KEY, Err("Cannot connect to the Docker daemon"));
    let error = docker_service::prune(&cli, &["containers".to_owned()]).unwrap_err();
    assert_eq!(error.code, ErrorCode::Io);
    assert!(error.message.contains("daemon"));
}

#[test]
fn prune_computes_freed_bytes_from_df_delta() {
    let after = concat!(
        r#"{"Type":"Images","TotalCount":"2","Active":"2","Size":"23.6GB","Reclaimable":"0B"}"#,
        "\n",
        r#"{"Type":"Containers","TotalCount":"1","Active":"1","Size":"0B","Reclaimable":"0B"}"#,
        "\n",
        r#"{"Type":"Local Volumes","TotalCount":"30","Active":"3","Size":"2.5GB","Reclaimable":"2.1GB (84%)"}"#,
        "\n",
        r#"{"Type":"Build Cache","TotalCount":"10","Active":"0","Size":"1.1GB","Reclaimable":"1GB (90%)"}"#,
    );
    let cli = MockCli::new()
        .push(DF_KEY, Ok(DF))
        .push(
            CONT_PRUNE_KEY,
            Ok("Deleted Containers:\nabc123def456\n\nTotal reclaimed space: 12MB\n"),
        )
        .push(
            IMAGE_PRUNE_KEY,
            Ok(concat!(
                "Deleted Images:\n",
                "untagged: nginx@sha256:aaa\n",
                "deleted: sha256:bbb\n\n",
                "Total reclaimed space: 1.2GB\n",
            )),
        )
        .push(DF_KEY, Ok(after));

    let result = docker_service::prune(
        &cli,
        &["containers".to_owned(), "images_dangling".to_owned()],
    )
    .expect("prune sukses");
    assert_eq!(result.freed_bytes, 1_212_300_000);
    assert_eq!(result.deleted, 3);
    assert!(result.output.contains("Deleted Containers"));
    assert!(result.output.contains("Deleted Images"));
}

#[test]
fn empty_targets_is_noop() {
    let result = docker_service::prune(&MockCli::new(), &[]).expect("noop");
    assert_eq!(result.freed_bytes, 0);
    assert_eq!(result.deleted, 0);
    assert!(result.output.is_empty());
}
