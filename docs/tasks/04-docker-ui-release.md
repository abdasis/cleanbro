# Task Breakdown: Docker, UI, Rilis

> Bagian dari [TASKS.md](../TASKS.md).

## Milestone M4: Adapter docker

### Task 4.1: Pembacaan status docker

**Files:** `apps/linux/src-tauri/src/adapters/docker.rs`

**Test:** `docker_adapter_parses_df_output` (fixture JSON),
`docker_adapter_reports_unavailable_without_binary`.

**Commit:** `feat: docker status adapter`.

### Task 4.2: Pratinjau dan eksekusi prune

**Files:** `apps/linux/src-tauri/src/adapters/docker.rs`,
`apps/linux/src/features/preview/docker-preview.tsx`

**Test:** `docker_prune_skips_volumes_in_use`,
`docker_prune_preview_lists_containers_before_running`.

**Verify manual:** jalankan pratinjau pada mesin dengan 3 container hidup,
pastikan container tersebut tidak pernah muncul sebagai target.

**Commit:** `feat: docker prune preview and execution`.

---

## Milestone M5: UI lengkap

### Task 5.1: App shell dan navigasi

**Files:** `apps/linux/src/app/layout/app-shell.tsx`, `apps/linux/src/app/layout/sidebar.tsx`
(masing-masing < 300 baris; pecah `nav-item.tsx` bila perlu)

**Verify:** semua layar dari UX.md dapat dibuka, fokus keyboard berfungsi.

**Commit:** `feat: app shell and navigation`.

### Task 5.2: Overview dan daftar kategori

**Files:** `apps/linux/src/features/scan/category-list.tsx`,
`apps/linux/src/features/scan/category-row.tsx`

**Verify:** ukuran cocok dengan hasil `du -sh` untuk 3 kategori.

**Commit:** `feat: overview category list`.

### Task 5.3: Preview sheet dan pemilihan item

**Files:** `apps/linux/src/features/preview/preview-sheet.tsx`,
`apps/linux/src/features/preview/item-table.tsx` (pakai virtualisasi bila > 200 item)

**Verify:** item `perlu tinjauan` default tidak terpilih; total berubah saat
pilihan berubah.

**Commit:** `feat: preview sheet with item selection`.

### Task 5.4: Dialog eksekusi, progress, hasil

**Files:** `apps/linux/src/features/execute/execute-dialog.tsx`,
`apps/linux/src/features/execute/progress-list.tsx`,
`apps/linux/src/features/execute/result-summary.tsx`

**Verify:** aksi 6 GB memaksa konfirmasi `HAPUS`; hasil menampilkan item gagal
beserta alasannya.

**Commit:** `feat: execution flow with progress and result`.

### Task 5.5: Audit tanpa emoji dan design token

**Files:** `scripts/audit-no-emoji.sh` (Python lebih disukai), `docs/UX.md`

**Verify:** `bun run audit:emoji` mengembalikan 0 temuan pada `apps/linux/src/`.

**Commit:** `chore: no-emoji audit script`.

---

## Milestone M6: Rilis

### Task 6.1: Profil build dan ukuran installer

**Files:** `apps/linux/src-tauri/Cargo.toml` (`[profile.release]`),
`apps/linux/src-tauri/tauri.conf.json` (`bundle`)

**Verify:** `bunx --bun tauri build` menghasilkan `.deb` < 25 MB; jalankan
binary hasil build dan lakukan satu scan penuh.

**Commit:** `build: release profile and bundle config`.

### Task 6.2: Release workflow

**Files:** `.github/workflows/release.yml`

**Verify:** tag `v0.1.0` menghasilkan draft release dengan 3 artifact OS.

**Commit:** `ci: release workflow with tauri-action`.

### Task 6.3: Uji lapangan dan kalibrasi

**Verify:** pada mesin uji dengan sisa < 5 GB, ukur dan catat: durasi scan,
ruang dibebaskan, deviasi estimasi terhadap `df`. Simpan hasil di
`docs/FIELD-TEST.md`.

**Commit:** `docs: field test results v0.1.0`.

---

## Definisi selesai v0.1

1. Tujuh acceptance criteria di SPEC.md bagian 8 terverifikasi dengan bukti
   output nyata, bukan asumsi.
2. Tidak ada file source melebihi 300 baris.
3. CI hijau di tiga OS.
4. Riwayat scan dan aksi dapat dibaca ulang setelah aplikasi ditutup dan dibuka.
