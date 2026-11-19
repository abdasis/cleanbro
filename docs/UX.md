# UX dan Design System: Sapu Bersih (Linux)

Dokumen ini berlaku untuk `apps/linux`. Aplikasi macOS CleanBro memakai tema
SwiftUI yang sudah ada (sidebar, split view, light/dark) dan tidak mengikuti
token di bawah ini.

## 1. Prinsip visual

1. Monokrom presisi: hitam `#000000` dan putih `#ffffff`, abu netral
   `neutral-800`/`neutral-900` untuk dark, `neutral-200` untuk border light.
2. Border hairline 1px, tanpa shadow tebal, tanpa gradient.
3. Font Geist Sans (UI) dan Geist Mono (angka, ukuran file, path).
4. Tanpa emoji di seluruh UI. Ikon hanya Lucide React, ukuran 16 atau 20 px,
   `stroke-width` 1.5.
5. Warna aksen hanya untuk status risiko: aman = `neutral`, perlu tinjauan =
   `amber-500` (border saja), berisiko = `red-500` (border + label). Tidak ada
   tombol berwarna terang.
6. Whitespace lega: padding panel 24 px, gap antar kartu 16 px, tinggi baris
   tabel 44 px.
7. Micro-interaction halus: transisi `opacity`/`border-color` 120 ms, tanpa
   scale, tanpa bounce.

Token Tailwind (dipakai konsisten, tidak ada warna hardcoded di komponen):

```ts
// tailwind.config.ts (ringkas)
colors: {
  background: "#ffffff", foreground: "#000000",
  muted: "#f5f5f5", "muted-foreground": "#737373",
  border: "#e5e5e5", card: "#ffffff",
  // dark
  "dark-background": "#0a0a0a", "dark-foreground": "#fafafa",
  "dark-border": "#262626", "dark-card": "#111111",
}
fontFamily: { sans: ["Geist Sans"], mono: ["Geist Mono"] }
```

## 2. Struktur layar

```text
+---------------------------------------------------------------+
| Titlebar (drag region)        [Scan cepat] [Scan dalam]  [gear]|
+-----------+---------------------------------------------------+
| Sidebar   |  Overview                                         |
| - Overview|  +---------------------------+------------------+   |
| - Scan    |  | Disk 187/233 GB  85%      | Sisa 35 GB       |   |
| - Preview |  | [progress bare netral]    | tren 7 hari      |   |
| - Riwayat |  +---------------------------+------------------+   |
| - Setelan |  Kategori                                          |
|           |  [ Cache developer   34.1 GB   aman    > ]       |
|           |  [ Docker            24.8 GB   review  > ]       |
+-----------+---------------------------------------------------+
```

### Layar 1: Overview

- Kartu disk: kapasitas, terpakai, sisa, persentase, badge peringatan.
- Daftar kategori hasil scan terakhir dengan ukuran mono dan badge risiko.
- Tombol utama: `Scan cepat` (primary, hitam), `Scan dalam` (outline).
- State kosong: teks "Belum ada pemindaian" + tombol scan, tanpa ilustrasi.

### Layar 2: Scan berjalan

- Progress bar indeterminate saat enumerasi, determinate saat agregasi.
- Baris status: jumlah file dipindai, kategori selesai, path aktif (mono,
  truncate dari kiri), tombol `Batal`.
- Hasil parsial ditandai badge `Sebagian` bila scan dibatalkan atau kena limit.

### Layar 3: Preview (sheet dari kanan, lebar 560 px)

- Header: nama kategori, total ukuran, badge risiko, rule id (mono, 12 px).
- Ringkasan: `N item, X GB akan dibebaskan`.
- Tabel item: checkbox, path (mono, truncate tengah), ukuran, alasan.
- Default semua terpilih hanya untuk risiko `aman`. Risiko `perlu tinjauan`
  dan `berisiko` default tidak terpilih.
- Footer sticky: `Pilih semua aman`, `Batal`, `Bersihkan (X GB)`.
- Tombol `Bersihkan` nonaktif bila tidak ada item terpilih.

### Layar 4: Dialog eksekusi

- Mode hapus: segmented control `Ke trash` (default) dan `Hapus permanen`.
- Bila total > 5 GB: field teks dengan label `Ketik HAPUS untuk melanjutkan`.
- Daftar peringatan guard (jika ada item dilewati) ditampilkan sebelum tombol
  aktif.
- Selama proses: daftar item dengan status `menunggu`, `berjalan`, `selesai`,
  `gagal`, plus counter byte.

### Layar 5: Hasil

- Rangkuman: dibebaskan, dihapus, gagal, dilewati, durasi.
- Tabel kegagalan dengan alasan (permission, guard, io).
- Aksi: `Undo` (bila semua item masih di trash), `Lihat riwayat`, `Selesai`.

### Layar 6: Riwayat

- Tabel: waktu, mode, jumlah item, dibebaskan, status, aksi.
- Filter: rentang tanggal, status, kategori.
- Undo per aksi dengan indikator sisa masa berlaku.
- State kosong: "Belum ada aksi pembersihan".

### Layar 7: Setelan

- Bahasa (Indonesia/Inggris).
- Mode default hapus (trash/permanen) dengan peringatan.
- Kategori yang diaktifkan.
- Lokasi log dan tombol `Buka folder log`.

## 3. Alur utama

```text
Start -> Overview -> [Scan cepat] -> Scan berjalan -> Overview terisi
     -> pilih kategori -> Preview -> atur pilihan -> [Bersihkan]
     -> Dialog eksekusi -> [konfirmasi] -> Progress -> Hasil -> [Undo]?
```

Aturan: tidak ada jalur yang mengeksekusi penghapusan tanpa melewati Preview
dan Dialog eksekusi. Tombol kombinasi keyboard tidak boleh melewati keduanya.

## 4. State dan feedback

| Kondisi | Perilaku UI |
| --- | --- |
| Scan berjalan | Tombol scan jadi `Batal`, sidebar tetap bisa dipakai |
| Scan batal | Badge `Sebagian`, kategori yang selesai tetap tampil |
| Rule tidak tersedia (misal docker absen) | Baris kategori dengan label `Tidak tersedia` + alasannya |
| Guard menolak saat eksekusi | Item masuk daftar `Dilewati` di layar hasil |
| Disk penuh saat scan | Banner `Sisa disk kritis` di atas konten, scan tetap jalan |
| Error IPC | Toast netral dengan kode error mono dan tombol `Coba lagi` |

Semua toast memakai Sonner dengan gaya netral: latar kartu, border hairline,
tanpa ikon emoji, ikon Lucide 16 px.

## 5. Keyboard dan aksesibilitas

- `Ctrl/Cmd+K` membuka command palette (scan, riwayat, setelan).
- `Ctrl/Cmd+R` memulai scan cepat; `Esc` menutup sheet/dialog.
- `Space` pada baris tabel memilih item; `Shift+Space` memilih rentang.
- Fokus terlihat: ring 2 px `neutral-900` (light) atau `neutral-100` (dark).
- Semua kontrol punya `aria-label`; progress memakai `role="progressbar"`
  dengan `aria-valuenow`.
- Kontras teks minimal 4.5:1; angka ukuran memakai mono agar sejajar.

## 6. Format angka dan teks

- Ukuran: `34.1 GB`, `557 MB`, satu desimal, pemisah titik, basis 1000.
- Tanggal: `19 Nov 2026, 14:32` (lokal), mono pada tabel.
- Path panjang: `truncate` tengah, tooltip berisi path penuh.
- Bahasa manusiawi untuk alasan: "Cache Gradle dibangun ulang otomatis saat
  build berikutnya", bukan "matched rule cache.gradle".

## 7. Larangan eksplisit

- Tanpa emoji dan emoticon, termasuk di label, chip, judul, dan toast.
- Tanpa warna brand terang, gradient, glassmorphism, atau shadow besar.
- Tanpa ilustrasi kartun; visual hanya data.
- Tanpa animasi lebih dari 200 ms, tanpa animasi pada progress bar.
- Tanpa modal bertumpuk lebih dari satu level.
