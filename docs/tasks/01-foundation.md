# Task Breakdown: Fondasi dan Scaffold

> Untuk Hermes: eksekusi memakai skill `subagent-driven-development`, satu
> subagent per task, review spesifikasi lalu review kualitas kode sebelum lanjut.

Goal: aplikasi desktop Tauri yang memindai dan membersihkan cache/artefak dengan
preview, konfirmasi, riwayat, dan undo.

Aturan umum untuk setiap task:

- Selesai berarti `cargo test`/`bun run test` hijau dan `git commit` sudah dibuat.
- Maksimum 300 baris per file; pecah sebelum melewati.
- Tidak menambah dependency tanpa menyebutkannya di deskripsi task.
- Semua path di bawah ini relatif terhadap root repo.

---

## Milestone M0: Scaffold (selesai)

### Task 0.1: Workspace monorepo

**Status:** selesai 2026-11-19.

- `Cargo.toml` root: workspace `crates/cleanbro-core` + `apps/linux/src-tauri`.
- `package.json` root: bun workspaces `apps/*`, `packages/*`; script
  `typecheck`, `dev:linux`, `build:linux`.
- `crates/cleanbro-core`: `error.rs`, `models.rs`, `rules.rs` + unit test.
- `apps/linux`: Tauri 2, identifier `com.abdasis.sapubersih`, window 1100x720
  (minimum 900x600), command `app_info`, port dev 1420.
- `packages/ui`: `AppShell` React dengan token monokrom light/dark.
- `apps/macos`: app SwiftUI dan CLI Go dipindahkan apa adanya.
- CI: `.github/workflows/ci.yml` (Rust + frontend Linux),
  `.github/workflows/build.yml` (macOS).

### Task 0.2: Verifikasi scaffold

- `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`,
  `cargo test --workspace`, `bun run typecheck` hijau.
- Build app (`tauri dev/build`) belum dijalankan di sesi scaffold.

### Task 0.3: Berikutnya

Tailwind 4 dan shadcn/ui sudah terpasang sebagai dependency; komponen shadcn
ditambahkan saat layar pertama dibangun (M5), bukan sebelum dibutuhkan.


---


---

## Milestone M1: Fondasi Rust

### Task 1.1: Model error dan tipe IPC

**Files:** `apps/linux/src-tauri/src/error.rs`, `apps/linux/src-tauri/src/models/mod.rs`

**Step 1:** Tulis test `error_serializes_with_stable_code` yang memastikan
`AppError { code: PermissionDenied }` menjadi `{"code":"PermissionDenied",...}`.

**Step 2:** Jalankan `cargo test error` dan pastikan gagal.

**Step 3:** Implementasikan `AppError`, `ErrorCode`, dan `impl Serialize`.

**Step 4:** `cargo test error` lulus. Commit `feat: stable ipc error model`.

### Task 1.2: Command `disk_summary`

**Files:** `apps/linux/src-tauri/src/commands/system.rs`, `apps/linux/src-tauri/src/lib.rs`,
`apps/linux/src-tauri/src/models/disk.rs`

**Step 1:** Test `disk_summary_reports_free_below_total` pada `fs2`/`statvfs`.

**Step 2:** Implementasi dengan `sysinfo::Disks`, hitung `warn_level`
(<10 persen kuning, <3 persen merah).

**Step 3:** Daftarkan command di `invoke_handler`, panggil dari
`apps/linux/src/features/overview/disk-card.tsx` dan tampilkan angka asli.

**Step 4:** Verifikasi manual: angka cocok dengan `df -h /`.

**Step 5:** Commit `feat: disk summary command and card`.

### Task 1.3: Store SQLite dan migrasi

**Files:** `apps/linux/src-tauri/src/store/db.rs`, `apps/linux/src-tauri/src/store/migrations.rs`

**Step 1:** Test `migrations_create_all_tables` memakai DB in-memory,
memeriksa keberadaan 6 tabel dari ARCHITECTURE.md bagian 7.

**Step 2:** Implementasi `rusqlite` bundled + `user_version` migration runner.

**Step 3:** Commit `feat: sqlite store with migrations`.

---

