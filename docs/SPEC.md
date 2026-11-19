# Spesifikasi Produk: Sapu Bersih

Versi: 1.1
Tanggal: 2026-11-19
Status: MVP v0.1, implementasi dimulai dari scaffold monorepo

## 0. Platform

Repo ini monorepo dengan dua aplikasi desktop:

| Folder | Aplikasi | Shell desktop | Status |
| --- | --- | --- | --- |
| `apps/linux` | Sapu Bersih | Tauri 2 (Rust + React 19) | Target v0.1, dikerjakan sekarang |
| `apps/macos` | CleanBro | SwiftUI (aplikasi existing) | Berjalan terpisah, tidak di-port ke Tauri |

Dokumen ini mendefinisikan Sapu Bersih untuk Linux. Parity fitur macOS dan
matriks platform ada di [PLATFORMS.md](PLATFORMS.md); katalog rule macOS ada di
[RULES.md](RULES.md) bagian 11.


## 1. Latar belakang

Kondisi nyata pada mesin kerja utama (Ubuntu 26.04, NVMe 233 GB):

- Disk terpakai 220 GB dari 233 GB, sisa 1.3 GB, filesystem 100 persen penuh.
- Penyebab utama bukan data pengguna, melainkan artefak yang bisa dibangun ulang:
  cache Gradle 11 GB, cache bun 9.5 GB, cache Cypress 5.3 GB, cache uv 5 GB,
  cache npm 3.5 GB, cache HuggingFace 2.1 GB, image docker tidak terpakai 2.5 GB,
  dan image docker aktif 24.8 GB.
- Pembersihan manual berhasil membebaskan 34 GB, tetapi prosesnya berisiko:
  harus tahu direktori mana yang aman, harus menghindari volume database, dan
  tidak ada catatan apa yang sudah dibuang.

Kesimpulan: kebutuhan sebenarnya bukan "penghapus file", melainkan
"penilai kelayakan hapus" yang aman, dapat diaudit, dan dapat diulang.

## 2. Tujuan

1. Menemukan ruang disk yang dapat dibebaskan dalam waktu kurang dari 60 detik
   pada mesin dengan lebih dari 1 juta file.
2. Menampilkan temuan per kategori dengan ukuran, alasan, dan tingkat risiko.
3. Menghapus hanya setelah pengguna melihat preview dan menyetujui.
4. Menjaga data proyek, database, dan konfigurasi dari penghapusan.
5. Mencatat setiap aksi sehingga bisa di-undo dan diaudit.

## 3. Non-goals

- Bukan uninstaller aplikasi sistem.
- Bukan alat partisi atau pengubah ukuran filesystem.
- Bukan pembersih lintas mesin atau agen remote (v1 lokal saja).
- Tidak menambal kekurangan disk dengan kompresi atau deduplikasi.
- Tidak menjalankan `docker system prune` otomatis tanpa konfirmasi pengguna.

## 4. Pengguna dan skenario

| Persona | Skenario | Hasil yang diharapkan |
| --- | --- | --- |
| Developer tunggal (pengguna utama) | Disk penuh saat build | Lihat kategori terbesar, pilih cache, bersihkan 30 GB dalam 2 menit |
| Developer dengan banyak proyek | Banyak `node_modules`, `target`, `.gradle` | Daftar artefak per proyek dengan opsi hapus selektif |
| Pengguna kantor | Downloads menumpuk, thumbnail besar | Rekomendasi aman tanpa menyentuh dokumen |
| Operator docker | Image dan container menumpuk | Pratinjau hasil prune docker sebelum dijalankan |

## 5. Ruang lingkup v0.1 (MVP)

Fitur wajib:

- FR-01 Ringkasan disk: total, terpakai, sisa, persentase, peringatan saat sisa
  kurang dari 10 persen.
- FR-02 Scan cepat: mengukur direktori kategori terdaftar dan menghitung ukuran
  rekursif dengan batas kedalaman serta pembatalan.
- FR-03 Scan dalam: menemukan artefak build per proyek (`node_modules`,
  `target`, `dist`, `.next`, `.gradle`), file besar (> 200 MB), dan file lama
  (> 90 hari) pada folder unduhan.
- FR-04 Preview per kategori: jumlah item, total ukuran, contoh 10 path teratas,
  tingkat risiko, dan catatan mengapa aman.
- FR-05 Eksekusi: hapus ke trash (default) atau hapus permanen (opt-in per aksi),
  dengan progress dan hasil per item.
- FR-06 Riwayat: tabel scan dan aksi, waktu, ukuran dibebaskan, status, dan
  tombol undo selama entri trash masih ada.
- FR-07 Aturan keamanan: allowlist rule, deny-list guard, konfirmasi teks untuk
  aksi di atas 5 GB, dan mode dry-run default di build debug.
- FR-08 Adapter docker: baca `docker system df` dan daftar image/container,
  tampilkan ruang yang bisa dibebaskan, jalankan prune hanya setelah konfirmasi.

Di luar v0.1 (lihat roadmap bagian 10): penjadwalan otomatis, tray/menu bar,
profil aturan kustom pengguna, pembandingan antar waktu, dan mode CLI.

## 6. Functional requirements (detail)

### FR-01 Ringkasan disk

- Membaca `statvfs` untuk filesystem tempat `$HOME` berada.
- Menampilkan: kapasitas, terpakai, sisa, persentase, dan tren 7 hari terakhir
  bila riwayat sudah ada.
- Ambang peringatan: sisa < 10 persen (kuning), sisa < 3 persen (merah).

### FR-02 Scan cepat

- Kategori yang dipindai: cache developer, cache browser, cache package manager,
  docker, log sistem, thumbnail, crash dump, trash.
- Target waktu: 20 detik untuk 500 ribu file pada SSD; batas keras 120 detik.
- Scan harus bisa dibatalkan kapan pun dan tetap mengembalikan hasil parsial
  yang ditandai `partial: true`.
- Tidak mengikuti symlink keluar dari `$HOME` kecuali rule menyatakan lain.

### FR-03 Scan dalam

- Artefak build per proyek: mengidentifikasi root proyek dari penanda
  (`package.json`, `Cargo.toml`, `composer.json`, `build.gradle`, `pubspec.yaml`)
  lalu melaporkan direktori artefak di dalamnya beserta ukurannya.
- File besar: kandidat > 200 MB di `$HOME`, dengan pengecualian direktori sistem
  dan direktori aplikasi.
- File lama: berkas di `~/Downloads` dan `~/Pictures/Wallpapers` yang tidak
  diakses lebih dari 90 hari, hanya sebagai saran, tidak pernah dipilih otomatis.

### FR-04 Preview

- Setiap kategori menampilkan: nama, jumlah item, total ukuran, risiko
  (`aman`, `perlu tinjauan`, `berisiko`), dasar aturan (rule id), dan 10 path
  terbesar.
- Pengguna dapat mengecualikan item individual sebelum eksekusi.
- Estimasi ruang yang dibebaskan dihitung ulang saat pilihan berubah.

### FR-05 Eksekusi

- Dua mode: `trash` (default, dapat di-undo) dan `delete` (permanen, butuh
  konfirmasi ganda dan hanya untuk kategori bertanda aman).
- Progress per item lewat event; UI menampilkan item aktif, jumlah selesai,
  gagal, dan byte yang dibebaskan.
- Kegagalan per item tidak menggagalkan seluruh aksi; hasil akhir merangkum
  `deleted`, `failed`, `skipped` beserta alasan.
- Sebelum eksekusi, setiap path divalidasi ulang terhadap rule dan guard
  (time-of-check to time-of-use). Path yang tidak lagi valid dilewati.

### FR-06 Riwayat dan undo

- Setiap scan dan aksi disimpan di SQLite lokal.
- Undo tersedia selama item masih ada di trash; statusnya ditandai `undoable`.
- Undo memulihkan item ke path asal dan mencatat hasilnya.

### FR-07 Aturan keamanan

- Deny-list absolut (tidak pernah boleh dihapus, bahkan oleh rule):
  `~/.ssh`, `~/.gnupg`, `~/.config`, `~/.local/share/keyrings`, `~/Documents`,
  `~/Pictures` (kecuali subfolder wallpaper yang eksplisit), setiap direktori
  yang berisi `*.db`, `*.sqlite`, `*.sql`, `pgdata`, `mysql`, `.env`,
  `.git` pada repo dengan perubahan belum di-commit.
- Path di luar `$HOME` hanya boleh disentuh lewat adapter dengan elevasi
  (`/var/cache/apt`, journald) dan selalu butuh konfirmasi eksplisit.
- Aksi di atas 5 GB mewajibkan pengguna mengetik `HAPUS` sebelum tombol aktif.
- Semua keputusan (rule cocok, guard gagal, alasan skip) masuk audit log.

### FR-08 Adapter docker

- Hanya membaca status dengan `docker system df --format json` dan
  `docker ps -a`; tidak memodifikasi apa pun saat scan.
- Aksi `prune` menampilkan pratinjau: daftar container mati, image tanpa
  referensi, volume tanpa container, dan total yang akan dibebaskan.
- Volume yang masih terpasang ke container hidup tidak pernah ditawarkan untuk
  dihapus, dan ditandai jelas sebagai "dipakai".

## 7. Non-functional requirements

| ID | Requirement | Target |
| --- | --- | --- |
| NFR-01 | Waktu start sampai UI interaktif | < 1.5 detik |
| NFR-02 | RAM saat idle | < 180 MB |
| NFR-03 | RAM saat scan 1 juta file | < 400 MB |
| NFR-04 | Ukuran installer | < 25 MB (Linux x64) |
| NFR-05 | Scan tidak boleh memblokir UI | 100 persen async, event-driven |
| NFR-06 | Tidak ada operasi jaringan | Aplikasi offline penuh |
| NFR-07 | Bahasa | Indonesia dan Inggris, i18n berbasis kamus |
| NFR-08 | Aksesibilitas | Fokus keyboard penuh, kontras AA, label ARIA |
| NFR-09 | Privilese | Berjalan tanpa root; elevasi hanya lewat pkexec per aksi |
| NFR-10 | Batas file source | Maksimum 300 baris per file (React maupun Rust) |

## 8. Acceptance criteria MVP

1. Pada mesin uji dengan sisa < 2 GB, scan cepat selesai < 30 detik dan
   menampilkan minimal 6 kategori dengan ukuran yang benar (diverifikasi manual
   dengan `du -sh`).
2. Menjalankan pembersihan cache Gradle, bun, npm, dan uv membebaskan ruang
   sebesar selisih `df` sebelum dan sesudah, dengan deviasi < 5 persen dari
   estimasi.
3. Semua item yang dihapus berstatus `undoable` di riwayat, dan undo
   memulihkan minimal satu file yang diverifikasi dengan `ls`.
4. Tidak ada path pada deny-list yang bisa dieksekusi walau dipaksa lewat
   pemilihan manual; dibuktikan oleh unit test guard.
5. Aksi di atas 5 GB tidak bisa dijalankan tanpa mengetik `HAPUS`.
6. Tidak ada emoji di seluruh UI; audit dengan pencarian regex karakter
   non-ASCII pada folder `apps/linux/src/`.
7. `cargo clippy -- -D warnings`, `cargo test`, `bun run typecheck`, dan
   `bun run test` semuanya lulus di CI.

## 9. Risiko dan mitigasi

| Risiko | Dampak | Mitigasi |
| --- | --- | --- |
| Rule salah menandai path penting | Kehilangan data | Allowlist sempit, deny-list guard, default trash, uji regresi per rule |
| Scan lambat pada disk HDD | UX buruk | Batas kedalaman, prune direktori sistem, progress streaming, tombol batal |
| Docker prune menghapus image yang masih dibutuhkan | Rebuild lama | Pratinjau wajib, volume terpakai dikecualikan, tanpa flag `-a` by default |
| Trash memenuhi disk | Pembersihan gagal | Deteksi kapasitas trash, tawarkan hapus permanen untuk isi trash lama |
| Perbedaan path antar OS | Bug portabilitas | Semua path lewat `PathBuf`, rule memakai pemisah netral, uji di tiga OS |

## 10. Roadmap

- v0.1 MVP: FR-01 sampai FR-08 untuk Linux (Tauri). macOS tetap di aplikasi
  CleanBro terpisah.
- v0.2: penjadwalan (cron internal + tray), profil aturan kustom, dukungan Windows.
- v0.3: mode CLI headless (`sapu-bersih scan --json`), integrasi notifikasi,
  pembanding tren disk 30 hari.

## 11. Pertanyaan terbuka

1. Apakah perlu pemindaian otomatis saat login (butuh opt-in jelas)?
2. Apakah undo harus menyimpan salinan penuh untuk file besar, atau cukup
   bergantung pada trash OS?
3. Apakah adapter docker perlu mendukung Podman pada v0.1?
