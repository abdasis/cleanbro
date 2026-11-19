# Matriks Platform

Status per 2026-11-19.

| Area | Linux (`apps/linux`, Sapu Bersih) | macOS (`apps/macos`, CleanBro) |
| --- | --- | --- |
| Shell desktop | Tauri 2, Rust + React 19 | SwiftUI (app existing) |
| Ringkasan disk | tersedia | tersedia |
| Scan cache developer | tersedia | tersedia (Xcode, npm, Gradle, `~/.cache`) |
| Scan log dan crash report | belum | tersedia |
| Preview sebelum hapus | tersedia | tersedia |
| Hapus ke trash | tersedia | tersedia |
| Riwayat dan undo | tersedia | tidak ada |
| Docker adapter | tersedia (status + prune aman) | tidak berlaku |
| Cache sistem (apt/journald) | target v0.1 | tidak berlaku |
| Homebrew cleanup | tidak berlaku | tersedia |
| Uninstall aplikasi | di luar v0.1 | tersedia |
| Disk usage tree | belum | tersedia |
| Large files | belum | tersedia |
| Stale files | belum | tersedia |
| Duplicate files | belum | tersedia |
| Tema terang/gelap | mengikuti sistem | tersedia |
| Bahasa | Indonesia + Inggris | Indonesia + Inggris |

## Aturan parity

- Fitur yang sudah ada di macOS menjadi acuan perilaku untuk Sapu Bersih, bukan
  acuan kode. Engine dan rule Rust hanya untuk Linux.
- Perbedaan platform wajib tercatat di tabel ini saat fitur ditambah atau
  dihapus.
- `crates/cleanbro-core` dan `packages/ui` tidak dipakai aplikasi macOS. Perubahan di
  sana tidak boleh mengasumsikan macOS memakainya.
