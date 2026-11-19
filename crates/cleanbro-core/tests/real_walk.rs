//! Ukur walker pada root nyata. Jalankan eksplisit:
//! `CLEANBRO_REAL_ROOT=$HOME cargo test -p cleanbro-core --test real_walk -- --ignored --nocapture`

use std::path::PathBuf;
use std::time::Instant;

use cleanbro_core::{walk, CancellationToken, WalkOptions, WalkResult};

#[test]
#[ignore = "menelusuri root nyata; jalankan eksplisit dengan --ignored"]
fn walks_real_root_and_reports_stats() {
    let root = std::env::var_os("CLEANBRO_REAL_ROOT")
        .map(PathBuf::from)
        .expect("set CLEANBRO_REAL_ROOT");
    assert!(root.is_dir(), "root bukan direktori: {}", root.display());

    let started = Instant::now();
    let stats: WalkResult = walk(&root, &WalkOptions::default(), &CancellationToken::new());
    let elapsed = started.elapsed();

    println!(
        "real_walk root={} bytes={} files={} dirs={} duration_ms={} partial={} cancelled={}",
        root.display(),
        stats.bytes,
        stats.files,
        stats.dirs,
        elapsed.as_millis(),
        stats.partial,
        stats.cancelled
    );

    assert!(stats.files > 0, "tidak ada berkas terbaca");
    assert!(stats.bytes > 0, "bytes nol");
}
