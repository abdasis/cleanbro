use rusqlite::Connection;

/// Tabel wajib menurut ARCHITECTURE bagian 7.
pub const TABLES: [&str; 6] = [
    "scans",
    "findings",
    "actions",
    "action_items",
    "audit",
    "undo_journal",
];

/// Migrasi berurutan; indeks + 1 menjadi `PRAGMA user_version`.
pub const MIGRATIONS: &[&str] = &[MIGRATION_001, MIGRATION_002];

const MIGRATION_001: &str = r#"
CREATE TABLE IF NOT EXISTS scans (
    id TEXT PRIMARY KEY,
    mode TEXT,
    started_at INTEGER,
    finished_at INTEGER,
    partial INTEGER,
    bytes_found INTEGER
);
CREATE TABLE IF NOT EXISTS findings (
    id TEXT PRIMARY KEY,
    scan_id TEXT,
    rule_id TEXT,
    path TEXT,
    bytes INTEGER,
    item_count INTEGER,
    risk TEXT,
    project TEXT,
    excluded INTEGER DEFAULT 0
);
CREATE TABLE IF NOT EXISTS actions (
    id TEXT PRIMARY KEY,
    scan_id TEXT,
    mode TEXT,
    started_at INTEGER,
    finished_at INTEGER,
    freed_bytes INTEGER,
    status TEXT,
    confirm_token TEXT
);
CREATE TABLE IF NOT EXISTS action_items (
    id TEXT PRIMARY KEY,
    action_id TEXT,
    path TEXT,
    bytes INTEGER,
    state TEXT,
    reason TEXT,
    trash_ref TEXT
);
CREATE TABLE IF NOT EXISTS audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER,
    scan_id TEXT,
    item_id TEXT,
    rule_id TEXT,
    decision TEXT,
    reason TEXT
);
CREATE TABLE IF NOT EXISTS undo_journal (
    action_id TEXT PRIMARY KEY,
    payload TEXT,
    created_at INTEGER,
    expires_at INTEGER
);
CREATE INDEX IF NOT EXISTS idx_findings_scan ON findings(scan_id);
CREATE INDEX IF NOT EXISTS idx_audit_scan ON audit(scan_id);
"#;

/// Penanda status excluded yang diubah pengguna (tri-state: belum disentuh).
const MIGRATION_002: &str = r#"
ALTER TABLE findings ADD COLUMN excluded_set INTEGER DEFAULT 0;
CREATE INDEX IF NOT EXISTS idx_actions_started ON actions(started_at);
CREATE INDEX IF NOT EXISTS idx_action_items_action ON action_items(action_id);
"#;

/// Jalankan migrasi yang belum pernah diterapkan pada koneksi ini.
pub fn migrate(connection: &mut Connection) -> rusqlite::Result<()> {
    let applied: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    for (index, sql) in MIGRATIONS.iter().enumerate() {
        let version = index as i64 + 1;
        if version <= applied {
            continue;
        }
        let transaction = connection.transaction()?;
        transaction.execute_batch(sql)?;
        transaction.pragma_update(None, "user_version", version)?;
        transaction.commit()?;
    }
    Ok(())
}

pub fn schema_version(connection: &Connection) -> rusqlite::Result<i64> {
    connection.pragma_query_value(None, "user_version", |row| row.get(0))
}

/// Daftar tabel yang ada, untuk pemeriksaan kesehatan dan uji migrasi.
pub fn existing_tables(connection: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )?;
    let rows = statement.query_map([], |row| row.get(0))?;
    rows.collect()
}
