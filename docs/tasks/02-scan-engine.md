# Task Breakdown: Engine Scan

> Bagian dari [TASKS.md](../TASKS.md).

## Milestone M2: Engine scan

### Task 2.1: Walker paralel dengan cancel

**Files:** `apps/linux/src-tauri/src/engine/walker.rs`, `apps/linux/src-tauri/src/engine/cancel.rs`

**Test:** `walker_respects_max_depth`, `walker_stops_on_cancel`,
`walker_matches_du_within_5_percent` (fixture 500 file).

**Implementasi:** `jwalk` + `CancellationToken`, jumlahkan `st_blocks * 512`
dengan fallback `len()`.

**Verify:** `cargo test engine::walker`.

**Commit:** `feat: parallel walker with cancellation`.

### Task 2.2: Katalog rule v1

**Files:** `apps/linux/src-tauri/src/rules/catalog.rs`, `apps/linux/src-tauri/src/rules/matcher.rs`,
`apps/linux/src-tauri/src/rules/tests/catalog_test.rs`

**Test:** satu test per rule ID dari RULES.md, plus
`matcher_rejects_unknown_path` dan `matcher_never_matches_denylist`.

**Implementasi:** `Rule` struct sesuai RULES.md bagian 10, `globset` matcher
dengan normalisasi `PathBuf`.

**Commit:** `feat: rule catalog v1 with matcher`.

### Task 2.3: Guard dan risiko

**Files:** `apps/linux/src-tauri/src/rules/guard.rs`, `apps/linux/src-tauri/src/rules/risk.rs`

**Test:** `guard_blocks_ssh_dir`, `guard_blocks_dirty_git_worktree`,
`guard_blocks_path_outside_home`, `guard_allows_gradle_cache`.

**Implementasi:** deny-list absolut, cek `git status --porcelain` (via
`git2` atau `std::process`) untuk artefak `dist`/`vendor`, cek ownership.

**Commit:** `feat: safety guard and risk classification`.

### Task 2.4: Command `start_scan` + event progress

**Files:** `apps/linux/src-tauri/src/services/scan_service.rs`,
`apps/linux/src-tauri/src/commands/scan.rs`

**Test:** `scan_service_emits_finished_event` memakai mock emitter.

**Implementasi:** `spawn_blocking`, emit `scan://progress` throttled 250 ms,
simpan hasil ke `scans` dan `findings`.

**Verify:** scan nyata pada `$HOME` selesai < 60 detik, hasil 8 kategori.

**Commit:** `feat: scan command with throttled progress events`.

---

