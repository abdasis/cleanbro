# CleanBro + Sapu Bersih

Monorepo berisi dua aplikasi pembersih disk untuk platform berbeda.

| Folder | Aplikasi | Stack | Platform |
| --- | --- | --- | --- |
| `apps/linux` | Sapu Bersih | Tauri 2, Rust, React 19 | Linux |
| `apps/macos` | CleanBro | SwiftUI + SwiftPM, CLI Go | macOS 14+ |

Keduanya memakai alur yang sama: pindai, tinjau, baru bersihkan. Tidak ada
penghapusan tanpa pratinjau.

## Struktur

```text
Cargo.toml             cargo workspace: crates/cleanbro-core, apps/linux/src-tauri
package.json           bun workspaces: apps/*, packages/*
crates/cleanbro-core/  tipe dan engine Rust (tanpa dependensi Tauri)
packages/ui/           AppShell React bersama
apps/linux/            Sapu Bersih (Tauri)
  src-tauri/           command Rust
  src/                 entry React
apps/macos/            CleanBro (SwiftUI existing)
  app/                 Swift package: Package.swift, Sources/, Icon/, build.sh
  cli/                 CLI Go cleanbro
  design/              mockup HTML
  tools/               skrip bantu (frame.swift)
docs/                  spesifikasi, arsitektur, rule, UX, task
```

## Sapu Bersih (Linux)

Prasyarat: Rust 1.99, bun 1.3+, dan dependensi Tauri untuk Linux
(`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`,
`librsvg2-dev`, `libxdo-dev`, `build-essential`).

```sh
bun install
bun run dev:linux      # tauri dev
bun run build:linux    # .deb + AppImage
```

Pemeriksaan:

```sh
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
bun run typecheck
```

Status: MVP v0.1. Tersedia: ringkasan disk, scan cepat/dalam, preview per
kategori, eksekusi ke trash dengan konfirmasi, riwayat + undo, adapter Docker
(status + prune aman), halaman Setelan, dan bahasa Indonesia/Inggris. Belum:
shortcut global, large/stale/duplicate files, dan rilis installer. Lihat
`docs/TASKS.md`.

## CleanBro (macOS)

Sama seperti sebelumnya:

```sh
cd apps/macos/app
./build.sh
open build/CleanBro.app
```

Instalasi satu baris, catatan Gatekeeper, izin Full Disk Access, dan build CLI
Go tidak berubah. Ringkasan:

- App: unduh `CleanBro.zip` dari release terbaru atau jalankan `install.sh`.
- CLI: `cd apps/macos/cli && go build -o bin/cleanbro .`
- Perlu macOS 14 (Sonoma) atau lebih baru dan Swift 6 (Xcode 16+).

Detail fitur macOS: `docs/PLATFORMS.md`.

## Dokumentasi

| Dokumen | Isi |
| --- | --- |
| `docs/SPEC.md` | Spesifikasi produk Sapu Bersih (Linux) |
| `docs/ARCHITECTURE.md` | Arsitektur teknis Linux |
| `docs/RULES.md` | Katalog rule Linux dan macOS |
| `docs/UX.md` | Design system Sapu Bersih |
| `docs/TASKS.md` | Milestone implementasi |
| `docs/PLATFORMS.md` | Matriks fitur per platform |

## CI

- `.github/workflows/ci.yml` — Rust workspace dan typecheck frontend Linux.
- `.github/workflows/build.yml` — CleanBro.app dan CLI Go macOS, plus release
  saat tag `v*`.

## Lisensi

GNU General Public License v3. Lihat `LICENSE`.
