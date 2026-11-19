mod common;

use std::time::Duration;

use cleanbro_core::{walk, CancellationToken, WalkOptions};

fn options(max_depth: usize) -> WalkOptions {
    WalkOptions {
        max_depth,
        max_files: 1000,
        deadline: Duration::from_secs(10),
    }
}

#[test]
fn walker_respects_max_depth() {
    let dir = common::TempDir::new("walk-depth");
    common::write_file(&dir.path.join("a.txt"), 10);
    common::write_file(&dir.path.join("sub/b.txt"), 10);
    common::write_file(&dir.path.join("sub/deep/c.txt"), 10);

    let token = CancellationToken::new();
    let shallow = walk(&dir.path, &options(1), &token);
    assert_eq!(shallow.files, 1);
    assert_eq!(shallow.dirs, 1);

    let deeper = walk(&dir.path, &options(2), &token);
    assert_eq!(deeper.files, 2);
    assert_eq!(deeper.dirs, 2);
}

#[test]
fn walker_stops_on_cancel() {
    let dir = common::TempDir::new("walk-cancel");
    for index in 0..50 {
        common::write_file(&dir.path.join(format!("sub/{index}.bin")), 8);
    }
    let token = CancellationToken::new();
    token.cancel();
    let result = walk(&dir.path, &options(4), &token);
    assert!(result.cancelled);
    assert!(result.partial);
    assert!(result.files < 50);
}

#[test]
fn walker_sums_file_size() {
    let dir = common::TempDir::new("walk-size");
    common::write_file(&dir.path.join("data.bin"), 10_000);
    let result = walk(&dir.path, &options(2), &CancellationToken::new());
    assert_eq!(result.files, 1);
    assert!(!result.partial);
    assert!(result.bytes >= 10_000, "bytes={}", result.bytes);
    // Batas atas masuk akal: blok 4 KiB membulatkan paling banyak 4 KiB.
    assert!(result.bytes < 10_000 + 4096 * 4, "bytes={}", result.bytes);
}

#[test]
fn walker_marks_partial_when_file_budget_hit() {
    let dir = common::TempDir::new("walk-budget");
    for index in 0..10 {
        common::write_file(&dir.path.join(format!("{index}.bin")), 4);
    }
    let options = WalkOptions {
        max_depth: 2,
        max_files: 5,
        deadline: Duration::from_secs(10),
    };
    let result = walk(&dir.path, &options, &CancellationToken::new());
    assert!(result.partial);
    assert_eq!(result.files, 5);
}
