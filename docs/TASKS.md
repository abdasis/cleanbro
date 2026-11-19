# Task Breakdown

Indeks eksekusi. Setiap milestone ada di file terpisah karena aturan batas
300 baris per file juga berlaku untuk dokumen kerja.

| File | Isi |
| --- | --- |
| [tasks/01-foundation.md](tasks/01-foundation.md) | M0 scaffold, M1 fondasi Rust (error, disk summary, SQLite) |
| [tasks/02-scan-engine.md](tasks/02-scan-engine.md) | M2 walker paralel, katalog rule, guard, command scan |
| [tasks/03-preview-execute.md](tasks/03-preview-execute.md) | M3 preview, eksekusi trash, konfirmasi, riwayat, undo |
| [tasks/04-docker-ui-release.md](tasks/04-docker-ui-release.md) | M4 adapter docker, M5 UI lengkap, M6 rilis |

Semua task di atas berlaku untuk `apps/linux`. Aplikasi macOS (`apps/macos`)
dikerjakan terpisah dengan SwiftUI; lihat [PLATFORMS.md](PLATFORMS.md).

## Status

| Milestone | Status |
| --- | --- |
| M0 scaffold monorepo (cargo workspace, core crate, app Tauri, UI shell, CI) | selesai |
| M1 fondasi Rust | belum dimulai |
| M2 scan engine | belum dimulai |
| M3 preview dan eksekusi | belum dimulai |
| M4 docker adapter | belum dimulai |
| M5 UI lengkap | belum dimulai |
| M6 rilis | belum dimulai |

## Cara eksekusi

> Untuk Hermes: pakai skill `subagent-driven-development`, satu subagent per
> task, review spesifikasi lalu review kualitas kode sebelum lanjut.

Aturan umum untuk setiap task:

- Selesai berarti `cargo test`/`bun run test` hijau dan `git commit` sudah dibuat.
- Maksimum 300 baris per file; pecah sebelum melewati.
- Tidak menambah dependency tanpa menyebutkannya di deskripsi task.
- Semua path relatif terhadap root repo.
- Urutan milestone wajib: M1, M2, M3, M4, M5, M6. Task di dalam milestone boleh
  dikerjakan paralel hanya bila tidak menyentuh file yang sama.

## Definisi selesai v0.1

1. Tujuh acceptance criteria di [SPEC.md](SPEC.md) bagian 8 terverifikasi
   dengan bukti output nyata, bukan asumsi.
2. Tidak ada file source melebihi 300 baris.
3. CI Linux hijau (rust + web). Build Swift macOS tetap hijau.
4. Riwayat scan dan aksi tetap terbaca setelah aplikasi ditutup dan dibuka.
