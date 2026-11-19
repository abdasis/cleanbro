use cleanbro_core::models::{Finding, ScanId};
use cleanbro_core::rules::RiskLevel;
use cleanbro_core::store::migrations::{self, TABLES};
use cleanbro_core::store::scan_repo::{save_scan, ScanRow};
use cleanbro_core::store::{open, open_in_memory};

#[test]
fn migrations_create_all_tables() {
    let connection = open_in_memory().expect("state memori");
    let tables = migrations::existing_tables(&connection).expect("daftar tabel");
    for expected in TABLES {
        assert!(
            tables.iter().any(|table| table == expected),
            "tabel {expected} tidak ada: {tables:?}"
        );
    }
    assert_eq!(
        migrations::schema_version(&connection).unwrap(),
        migrations::MIGRATIONS.len() as i64
    );
}

#[test]
fn migrations_are_idempotent() {
    let mut connection = open_in_memory().expect("state memori");
    migrations::migrate(&mut connection).expect("migrasi ulang");
    assert_eq!(
        migrations::schema_version(&connection).unwrap(),
        migrations::MIGRATIONS.len() as i64
    );
}

#[test]
fn save_scan_and_findings() {
    let mut connection = open_in_memory().expect("state memori");
    let scan_id = ScanId::new();
    let row = ScanRow {
        id: scan_id.as_str().to_owned(),
        mode: "quick".to_owned(),
        started_at: 1,
        finished_at: 2,
        partial: false,
        bytes_found: 4096,
    };
    let finding = Finding::new(
        scan_id.clone(),
        "cache.gradle",
        "/home/uji/.gradle/caches",
        4096,
        4,
        RiskLevel::Safe,
        None,
    );
    save_scan(&mut connection, &row, std::slice::from_ref(&finding)).expect("simpan scan");

    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM findings", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
    let risk: String = connection
        .query_row("SELECT risk FROM findings", [], |row| row.get(0))
        .unwrap();
    assert_eq!(risk, "safe");
}

#[test]
fn open_creates_parent_directory_and_file() {
    let dir = std::env::temp_dir().join(format!("cleanbro-db-{}", uuid::Uuid::new_v4()));
    let path = dir.join("nested/state.db");
    let connection = open(&path).expect("buka state");
    drop(connection);
    assert!(path.is_file());
    let _ = std::fs::remove_dir_all(&dir);
}
