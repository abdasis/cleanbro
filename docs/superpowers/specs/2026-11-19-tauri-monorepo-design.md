# Desain: Monorepo CleanBro + Sapu Bersih

Tanggal: 2026-11-19
Status: disetujui, dengan revisi arah 2026-11-19

## Revisi arah

macOS tetap memakai aplikasi SwiftUI yang sudah ada (bukan di-port ke Tauri).
Tauri/Rust hanya untuk `apps/linux`. `crates/cleanbro-core` dan `packages/ui` dipakai
aplikasi Linux.

## Intent

Satu repo berisi dua aplikasi pembersih disk:

- `apps/linux` — Sapu Bersih, Tauri 2 (Rust + React 19), mengikuti spesifikasi
  repo `abdasis/sapu-bersih`.
- `apps/macos` — CleanBro, aplikasi SwiftUI existing + CLI Go, dipindahkan dari
  root repo tanpa perubahan perilaku.

Repo tujuan: `github.com/abdasis/cleanbro`. `LICENSE` GPL-3 tetap.

## Keputusan yang sudah diambil

| Topik | Keputusan | Alasan |
| --- | --- | --- |
| Cakupan rilis ini | Scaffold + spesifikasi. Satu command contoh (`app_info`) untuk Linux. Fitur belum diimplementasi. | Diminta eksplisit. |
| Pembagian kode | Cargo workspace + satu package UI React, khusus Linux. | Bug inti diperbaiki sekali; macOS tidak memakai stack ini. |
| macOS | SwiftUI existing dipertahankan. | Diminta eksplisit. |
| Push | Tambah monorepo Linux ke `main`, kode Swift dan CLI tetap ada di `apps/macos`. | Tidak ada yang dihapus. |

## Alternatif yang ditolak

- **Core bersama, frontend terpisah** — fitur Linux (docker, apt) dan macOS
  (Homebrew, uninstall) memang beda, tetapi 80% layar sama (disk summary,
  scan, preview, history). Duplikasi UI tidak sepadan.
- **Dua app terpisah total** — semua perbaikan dua kali, tanpa manfaat.

## Struktur

```text
cleanbro/
  Cargo.toml                     # workspace: crates/cleanbro-core, apps/linux/src-tauri
  package.json                   # bun workspaces: apps/*, packages/*
  crates/cleanbro-core/                   # engine scan, rules, guard, trash, store (skeleton)
  packages/ui/                   # React 19 + Tailwind 4, App shell (Linux)
  apps/linux/                    # Tauri app Sapu Bersih
    src-tauri/                   # shell tipis + command app_info
    src/                         # entry React, mount <AppShell platform="linux">
    tauri.conf.json
  apps/macos/                    # CleanBro SwiftUI existing
    app/                         # Package.swift, Sources/, Icon/, build.sh
    cli/                         # CLI Go cleanbro
    design/                      # mockup HTML
  docs/
    SPEC.md                      # spesifikasi produk Sapu Bersih (Linux)
    ARCHITECTURE.md              # arsitektur teknis Linux
    RULES.md                     # katalog rule Linux + macOS
    UX.md                        # design token dan layar Linux
    TASKS.md                     # milestone implementasi
    PLATFORMS.md                 # matriks fitur per platform
    superpowers/specs/           # dokumen ini
  .github/workflows/
    ci.yml                       # rust + web (Linux)
    build.yml                    # Swift app + CLI Go (macOS)
```

`crates/cleanbro-core` berisi modul bertipe jelas (`error`, `models`, `rules`) yang
dipakai `apps/linux`; modul engine/store ditambah saat implementasi, bukan
sekarang.

## Stack

- Linux: Rust 1.99, edition 2021, Tauri 2 stabil, `serde`, `thiserror`.
  React 19, TypeScript 5, Vite 7, Tailwind 4, bun.
- macOS: Swift 6 + SwiftPM (existing), CLI Go (existing).
- Nama produk: `Sapu Bersih`. Identifier: `com.abdasis.sapubersih`.

## Batas scaffold (definisi selesai)

1. `cargo check --workspace` dan `cargo clippy --workspace -- -D warnings` lulus.
2. `bun run typecheck` lulus untuk `packages/ui` dan `apps/linux`.
3. App Linux punya command `app_info` (versi, platform, `rules_version`),
   terbukti lewat unit test Rust.
4. `docs/` memuat spesifikasi: FR Linux, parity fitur macOS, katalog rule kedua
   platform, matriks platform, dan task breakdown.
5. `apps/macos` berisi app SwiftUI dan CLI Go yang sama seperti sebelumnya.
6. `README.md` menjelaskan struktur, cara dev, dan cara build.
7. CI Linux (`ci.yml`) hijau; workflow macOS lama tetap berjalan.
8. Scaffold sudah di-push ke `origin/main`.

## Di luar scaffold

Implementasi scan engine, preview, eksekusi, riwayat, docker adapter, dan rilis
installer Linux. Semua tercatat di `docs/TASKS.md`, dikerjakan milestone
terpisah.

## Catatan

- Build (`bunx tauri dev/build`) tidak dijalankan di sesi ini; hanya type
  check dan `cargo check`.
- Kode Swift dan CLI Go tidak diubah, hanya dipindahkan ke `apps/macos`.

