# Matriks Platform

Status per 2026-11-19.

| Area | Linux (`apps/linux`, Sapu Bersih) | macOS (`apps/macos`, CleanBro) |
| --- | --- | --- |
| Shell desktop | Tauri 2, Rust + React 19 | SwiftUI (app existing) |
| Ringkasan disk | target v0.1 | tersedia |
| Scan cache developer | target v0.1 | tersedia (Xcode, npm, Gradle, `~/.cache`) |
| Scan log dan crash report | target v0.1 | tersedia |
| Preview sebelum hapus | target v0.1 | tersedia |
| Hapus ke trash | target v0.1 | tersedia |
| Riwayat dan undo | target v0.1 | tidak ada |
| Docker adapter | target v0.1 | tidak berlaku |
| Cache sistem (apt/journald) | target v0.1 | tidak berlaku |
| Homebrew cleanup | tidak berlaku | tersedia |
| Uninstall aplikasi | di luar v0.1 | tersedia |
| Disk usage tree | tidak ada | tersedia |
| Large files | target v0.1 | tersedia |
| Stale files | target v0.1 | tersedia |
| Duplicate files | roadmap | tersedia |
| Tema terang/gelap | target v0.1 | tersedia |
| Bahasa | Indonesia + Inggris (target) | Indonesia + Inggris |

## Aturan parity

- Fitur yang sudah ada di macOS menjadi acuan perilaku untuk Sapu Bersih, bukan
  acuan kode. Engine dan rule Rust hanya untuk Linux.
- Perbedaan platform wajib tercatat di tabel ini saat fitur ditambah atau
  dihapus.
- `crates/cleanbro-core` dan `packages/ui` tidak dipakai aplikasi macOS. Perubahan di
  sana tidak boleh mengasumsikan macOS memakainya.
