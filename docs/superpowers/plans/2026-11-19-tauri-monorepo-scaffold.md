# Monorepo Tauri CleanBro — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Scaffold monorepo dengan `apps/linux` (Sapu Bersih, Tauri 2) dan `apps/macos` (CleanBro, SwiftUI existing).

**Architecture:** Cargo workspace di root (`crates/cleanbro-core` + dua `src-tauri`), bun workspace (`apps/*`, `packages/ui`). Setiap app adalah shell tipis: satu command `app_info` yang memanggil tipe bersama di `crates/cleanbro-core`, lalu React mount `AppShell` dari `packages/ui`.

**Tech Stack:** Rust 1.99 (edition 2021), Tauri 2, serde, thiserror; React 19, TypeScript 5, Vite 7, Tailwind 4, bun; GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-11-19-tauri-monorepo-design.md`

## Global Constraints

- Nama produk: `Sapu Bersih` (Linux). Identifier: `com.abdasis.sapubersih`. macOS tetap CleanBro SwiftUI.
- Semua nama file/folder English kebab-case; identifier code English; komentar boleh Indonesia.
- Tauri 2 stabil (`tauri = "2"`, `tauri-build = "2"`), bukan 3 alpha.
- Maksimum 300 baris per file source (SPEC NFR-10).
- Tanpa emoji di UI. Ikon hanya Lucide nanti.
- `docs/` wajib gabungan Linux + macOS.
- Build app (`bunx tauri dev/build`) tidak dijalankan; verifikasi = `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `bun run typecheck`.

## Review Focus

- `app_info` harus melaporkan platform dan versi yang benar per app; salah label tidak terlihat sampai UI jalan.
- `frontendDist` harus menunjuk folder yang ada saat `cargo check`, jika tidak compile gagal di CI bersih.
- Bun workspace `workspace:*` harus resolve tanpa `node_modules` global.
- Path rule/deny-list nanti harus lewat `PathBuf`, bukan string separator; scaffold tidak boleh mengunci pola string.
- CI di dua OS: dependency Linux (webkit2gtk-4.1) tidak ada di macOS — job harus dipisah.

---

### Task 1: Workspace + crates/cleanbro-core

**Files:**
- Create: `Cargo.toml`, `crates/cleanbro-core/Cargo.toml`, `crates/cleanbro-core/src/lib.rs`, `crates/cleanbro-core/src/error.rs`, `crates/cleanbro-core/src/models.rs`, `crates/cleanbro-core/src/rules.rs`
- Create: `package.json`, `.gitignore` (modify existing)

**Interfaces:**
- Consumes: none.
- Produces:
  - `cleanbro_core::AppInfo { name: String, version: String, platform: Platform, rules_version: String }` + `AppInfo::new(name: &str, version: &str, platform: Platform) -> AppInfo`
  - `Platform::{Linux, Macos}` (serde lowercase)
  - `RULES_VERSION: &str = "rules-v1"`
  - `AppError { code: ErrorCode, message: String, detail: Option<String> }`, `ErrorCode` 8 varian dari ARCHITECTURE, `AppResult<T>`
  - `RiskLevel::{Safe, Review, Risky}`, `DeleteMode::{Trash, Permanent}`

- [ ] **Step 1: Write failing tests** in `crates/cleanbro-core/src/models.rs` and `error.rs`:
  - `AppInfo::new("Sapu Bersih", "0.1.0", Platform::Linux)` → `platform == Platform::Linux`, `rules_version == "rules-v1"`.
  - serde JSON `AppInfo` memuat `"platform":"linux"` dan `"rules_version":"rules-v1"`.
  - `AppError` serialize menghasilkan `{"code":"GuardRejected","message":"...","detail":null}`.
- [ ] **Step 2: Run** `cargo test -p cleanbro-core` → FAIL (crate belum ada / tipe belum ada).
- [ ] **Step 3: Implement** workspace root (`members = ["crates/cleanbro-core", "apps/linux/src-tauri"]`, `resolver = "2"`, release profile `strip = true`, `lto = "thin"`, `opt-level = "s"`), core crate (`serde`, `thiserror`), dan tipe di atas.
- [ ] **Step 4: Run** `cargo test -p cleanbro-core` → PASS.
- [ ] **Step 5: Root `package.json`** dengan `"private": true`, `"workspaces": ["apps/*", "packages/*"]`, script `typecheck` menjalankan typecheck ketiga package.
- [ ] **Step 6: `.gitignore`** tambah `node_modules/`, `target/`, `dist/assets/`, `dist/*.map`; hapus entri Swift/Go setelah Task 6.
- [ ] **Step 7: Commit** `chore(workspace): tambah rust workspace dan crate core`.

---

### Task 2: apps/linux (Sapu Bersih)

**Files:**
- Create: `apps/linux/package.json`, `apps/linux/index.html`, `apps/linux/vite.config.ts`, `apps/linux/tsconfig.json`, `apps/linux/src/main.tsx`
- Create: `apps/linux/src-tauri/Cargo.toml`, `build.rs`, `src/main.rs`, `src/lib.rs`, `tauri.conf.json`, `capabilities/default.json`

**Interfaces:**
- Consumes: `cleanbro_core::{AppInfo, Platform, RULES_VERSION}`.
- Produces: command Tauri `app_info() -> AppInfo`; fungsi murni `fn current_app_info() -> AppInfo` (dites tanpa runtime Tauri).

- [ ] **Step 1: Failing test** di `apps/linux/src-tauri/src/lib.rs`: `current_app_info().name == "Sapu Bersih"`, `platform == Platform::Linux`, `version == env!("CARGO_PKG_VERSION")`.
- [ ] **Step 2: Run** `cargo test -p sapu-bersih` → FAIL.
- [ ] **Step 3: Implement** crate `sapu-bersih` (lib + bin tipis `main.rs` memanggil `run()`), `tauri.conf.json` dengan `productName: "Sapu Bersih"`, identifier `com.abdasis.sapubersih`, `frontendDist: "../dist"`, `devUrl: "http://localhost:1420"`, bundle target `["deb","appimage"]`, window 1100x720. Command `app_info` di `invoke_handler`.
- [ ] **Step 4: Run** `cargo test -p sapu-bersih` dan `cargo check -p sapu-bersih` → PASS.
- [ ] **Step 5: Commit** `feat(linux): scaffold app tauri sapu bersih`.

---

### Task 3: Pindahkan app macOS ke apps/macos (revisi)

Arah berubah: macOS tetap SwiftUI, bukan Tauri.

**Files:**
- Move: Swift package (`Package.swift`, `Sources/`, `Icon/`, `build.sh`) → `apps/macos/app/`; `main.go` + `go.mod` → `apps/macos/cli/`; `template/` → `apps/macos/design/`
- Modify: `.github/workflows/build.yml` (path build/cache/artifact)

- [ ] **Step 1:** `git mv` ketiga path, tanpa mengubah isi source.
- [ ] **Step 2:** Update path di workflow macOS; `build.sh` memakai `cd "$(dirname "$0")"` sehingga tetap jalan.
- [ ] **Step 3:** Verifikasi `swift build` tidak dijalankan (tidak ada macOS); cukup cek path workflow.
- [ ] **Step 4: Commit** `chore(macos): pindahkan app swift dan cli ke apps/macos`.

---

### Task 4: packages/ui + entry React

**Files:**
- Create: `packages/ui/package.json`, `tsconfig.json`, `src/index.ts`, `src/app-shell.tsx`, `src/styles.css`
- Modify: `apps/linux/src/main.tsx`, `apps/linux/vite.config.ts`, `apps/linux/tsconfig.json`

**Interfaces:**
- Produces: `AppShell({ product, platform, tagline }: { product: string; platform: "linux" | "macos"; tagline: string })`; package `@cleanbro/ui` dengan exports `"."` dan `"./styles.css"`.

- [ ] **Step 1: Implement** `AppShell`: header monokrom (judul produk, badge platform, tagline), area konten kosong dengan teks "Belum ada data". Tailwind 4 via `@tailwindcss/vite` di tiap app; token warna netral di `styles.css`.
- [ ] **Step 2: Entry app** mount `AppShell` dengan produk masing-masing; port dev Linux 1420, macOS 1421.
- [ ] **Step 3: Run** `bun install` lalu `bun run typecheck` → PASS.
- [ ] **Step 4: Verifikasi** `bun run typecheck` hijau; build app tidak dijalankan di sesi scaffold.
- [ ] **Step 5: Commit** `feat(ui): tambah app shell bersama dan entry dua aplikasi`.

---

### Task 5: Dokumentasi gabungan

**Files:**
- Create: `docs/SPEC.md`, `docs/ARCHITECTURE.md`, `docs/RULES.md`, `docs/UX.md`, `docs/TASKS.md`, `docs/PLATFORMS.md`
- Delete: `docs/screenshots/` (milik app Swift lama)

**Isi wajib:**
- SPEC: FR-01..FR-08 dari sapu-bersih + section "Parity macOS" (Home, Cleanup, Uninstall Apps, Homebrew, Disk Usage, Large Files, Stale, Duplicates, appearance, i18n EN/ID) + non-goals.
- ARCHITECTURE: model proses, struktur monorepo (`apps/*`, `crates/cleanbro-core`, `packages/ui`), kontrak IPC (tabel command + event), engine scan, rules/guard, store SQLite, build matrix.
- RULES: tabel sapu-bersih `rules-v1` + rule macOS (`macos.xcode.derived_data` = `~/Library/Developer/Xcode/DerivedData`, `macos.xcode.device_support`, `macos.xcode.simulator_caches`, `macos.cache.user` = `~/Library/Caches/**` guard, `macos.logs`, `macos.crash_reports`, `macos.trash`, `macos.homebrew.cleanup` = `brew cleanup`, `macos.uninstall.app_bundle`) dengan risiko dan deny-list macOS (`~/Library/Keychains`, `~/Library/Mobile Documents`, `~/Documents`, `~/Pictures`).
- TASKS: milestone M0 (scaffold, selesai) sampai M6 (rilis), path file, definisi selesai.
- PLATFORMS: matriks fitur Linux vs macOS (tersedia / menyusul / tidak berlaku).
- UX: token monokrom, tipografi Geist, aturan tanpa emoji, layout shell, i18n key English.

- [ ] **Step 1: Tulis enam dokumen** (boleh mengangkat isi sapu-bersih, disunting untuk monorepo).
- [ ] **Step 2: Verifikasi** tidak ada referensi `src-tauri/` lama tanpa prefix `apps/<platform>/`; setiap rule punya baris guard/risiko.
- [ ] **Step 3: Commit** `docs: satukan spesifikasi sapu bersih dan cleanbro`.

---

### Task 6: Hapus isi lama + README + CI

**Files:**
- Delete: `CleanBro/`, `main.go`, `go.mod`, `install.sh`, `template/`, `.github/workflows/build.yml`
- Create: `README.md` (replace), `.github/workflows/ci.yml`

- [ ] **Step 1: Hapus** folder/file lama. Simpan `LICENSE` (GPL-3).
- [ ] **Step 2: README baru**: apa itu dua app, struktur repo, prasyarat (Rust 1.99, bun, deps Linux), perintah dev/build per app, tautan docs, lisensi, catatan bahwa app Swift/CLI Go lama ada di riwayat git.
- [ ] **Step 3: CI** job `rust` (ubuntu-latest + macos-latest): install deps webkit (ubuntu), `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`; job `web` (ubuntu): `bun install --frozen-lockfile`, `bun run typecheck`.
- [ ] **Step 4: Verifikasi lokal**: `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`, `bun run typecheck`.
- [ ] **Step 5: Commit** `chore: ganti isi repo dengan monorepo tauri`.

---

### Task 7: Push

- [ ] **Step 1:** Review `git status` — tidak ada file biner besar, tidak ada secret.
- [ ] **Step 2:** `git push origin main`.
- [ ] **Step 3:** Verifikasi `git ls-remote origin -h refs/heads/main` sama dengan commit lokal.
