# Katalog Rule Pembersihan

Versi katalog: `rules-v1`. Setiap perubahan pola wajib menaikkan versi dan
menambah uji regresi.

Konvensi kolom:

- **Risiko**: `aman` (dibangun ulang otomatis), `perlu tinjauan` (mungkin
  berisi data pengguna), `berisiko` (hanya boleh dipilih manual satu per satu).
- **Mode hapus**: `trash` (default), `delete` (perlu konfirmasi ganda).
- **Contoh**: ukuran terukur pada mesin uji (Ubuntu 26.04, NVMe 233 GB).

## 1. Cache developer

| Rule ID | Pola path (relatif `$HOME`) | Risiko | Mode | Contoh |
| --- | --- | --- | --- | --- |
| `cache.gradle` | `.gradle/caches` | aman | trash | 11.0 GB |
| `cache.bun` | `.bun/install/cache` | aman | trash | 9.5 GB |
| `cache.npm` | `.npm/_cacache` | aman | trash | 3.5 GB |
| `cache.pnpm` | `.local/share/pnpm/store` | aman | trash | 1.4 GB |
| `cache.pip` | `.cache/pip` | aman | trash | 557 MB |
| `cache.uv` | `.cache/uv`, `.local/share/uv` | aman | trash | 6.4 GB |
| `cache.composer` | `.cache/composer` | aman | trash | 326 MB |
| `cache.cargo.registry` | `.cargo/registry/cache` | aman | trash | — |
| `cache.go` | `go/pkg/mod/cache/download` | aman | trash | — |

Guard: `cache.cargo.registry` tidak boleh menyertakan `.cargo/bin`.

## 2. Cache alat uji dan model

| Rule ID | Pola path | Risiko | Mode | Contoh |
| --- | --- | --- | --- | --- |
| `cache.cypress` | `.cache/Cypress` | aman | trash | 5.3 GB |
| `cache.playwright` | `.cache/ms-playwright` | aman | trash | 2.0 GB |
| `cache.huggingface` | `.cache/huggingface/hub` | aman | trash | 2.1 GB |
| `cache.dotslash` | `.cache/dotslash` | aman | trash | 395 MB |

Guard: `cache.huggingface` hanya `hub`, tidak `datasets` bila pengguna menandai
dataset sebagai aset tetap (v0.2).

## 3. Cache browser dan desktop

| Rule ID | Pola path | Risiko | Mode | Contoh |
| --- | --- | --- | --- | --- |
| `cache.chrome` | `.cache/google-chrome/*/Cache` | aman | trash | 699 MB |
| `cache.thumbnails` | `.cache/thumbnails` | aman | trash | — |
| `cache.tracker` | `.cache/tracker3` | aman | trash | 622 MB |

Guard: dilarang menyentuh `.config/google-chrome` (profil, bukan cache).

## 4. Artefak build per proyek

| Rule ID | Pola | Risiko | Mode |
| --- | --- | --- | --- |
| `build.node_modules` | `**/node_modules` di bawah root proyek | aman | trash |
| `build.rust_target` | `**/target` bila ada `Cargo.toml` | aman | trash |
| `build.next` | `**/.next` | aman | trash |
| `build.dist` | `**/dist` bila ada `package.json` | perlu tinjauan | trash |
| `build.gradle` | `**/build` bila ada `build.gradle*` | aman | trash |
| `build.flutter` | `**/.dart_tool`, `**/build` bila ada `pubspec.yaml` | aman | trash |
| `build.vendor` | `**/vendor` bila ada `composer.json` | perlu tinjauan | trash |

Guard: `build.dist` dan `build.vendor` dilewati bila direktori merupakan
worktree git dengan perubahan belum di-commit; `.next/cache` aman, `.next/standalone`
tidak ditawarkan karena bisa jadi artefak deploy.

## 5. Keluaran render dan media

| Rule ID | Pola | Risiko | Mode | Contoh |
| --- | --- | --- | --- | --- |
| `media.render_out` | `~/<proyek>/out`, `~/<proyek>/render` | perlu tinjauan | trash | 3.5 GB |
| `media.cache` | `~/<proyek>/cache` bila ada `package.json` | perlu tinjauan | trash | 982 MB |

Aturan khusus: direktori `public` dan `assets` tidak pernah ditawarkan karena
sering menjadi sumber input, bukan keluaran.

## 6. Docker (adapter, bukan filesystem)

| Rule ID | Aksi | Risiko | Catatan |
| --- | --- | --- | --- |
| `docker.containers_stopped` | `docker container prune -f` | aman | container mati saja |
| `docker.images_dangling` | `docker image prune -f` | aman | tanpa `-a` |
| `docker.images_unused` | `docker image prune -a -f` | perlu tinjauan | pratinjau wajib |
| `docker.volumes_unused` | `docker volume prune -f` | berisiko | volume terpasang dikecualikan |

Contoh terukur: image aktif 24.8 GB (3 image), volume 2.5 GB (30 volume,
3 terpakai). Penghapusan image yang dipakai container hidup ditolak adapter.

## 7. Sistem (butuh elevasi)

| Rule ID | Aksi | Risiko | Mekanisme |
| --- | --- | --- | --- |
| `system.apt_cache` | `apt-get clean` | aman | pkexec |
| `system.journal` | `journalctl --vacuum-size=200M` | aman | pkexec |
| `system.old_kernels` | daftar kernel tidak aktif | berisiko | tidak diotomatiskan di v0.1 |

Setiap aksi sistem menampilkan perintah persis yang akan dijalankan dan
meminta konfirmasi terpisah. Tanpa pkexec tersedia, rule berstatus
`unavailable` dengan petunjuk manual.

## 8. Saran, bukan aksi otomatis

| Rule ID | Pola | Perilaku |
| --- | --- | --- |
| `advice.big_files` | file > 200 MB di `$HOME` | tampil sebagai daftar, tidak pernah terpilih otomatis |
| `advice.old_downloads` | `~/Downloads/**` mtime > 90 hari | tampil dengan tanggal akses |
| `advice.trash` | isi trash > 30 hari | tawaran kosongkan trash |

## 9. Deny-list global

Tidak ada rule yang boleh mengalahkan daftar ini:

```text
~/.ssh/**              ~/.gnupg/**           ~/.config/**
~/.local/share/keyrings/**
~/.password-store/**
~/Documents/**         ~/Pictures/**         ~/Videos/** (kecuali pola eksplisit)
~/.hermes/**           ~/.claude/**
**/*.db                **/*.sqlite           **/*.sql
**/pgdata/**           **/mysql/**           **/.env
**/.git/**             (pada repo dengan perubahan belum di-commit)
/                      /boot  /etc  /usr  /var/lib  /home
```

Pengecualian hanya untuk `~/Pictures/Wallpapers` ketika rule wallpaper dipakai,
dan itu pun butuh pemilihan manual per file.

## 10. Format definisi rule

```rust
pub struct Rule {
    pub id: RuleId,                 // "cache.gradle"
    pub globs: &'static [&'static str],
    pub category: Category,         // DeveloperCache, Docker, Build, Media, System
    pub risk: RiskLevel,            // Safe, Review, Risky
    pub delete_mode: DeleteMode,    // Trash, Permanent
    pub requires: GuardSet,         // penanda proyek, kepemilikan, dsb
    pub max_depth: u8,
    pub note: &'static str,         // alasan manusiawi untuk UI
}
```

Menambah rule tanpa entri uji di `apps/linux/src-tauri/src/rules/tests/` akan gagal CI.

## 11. Katalog macOS (aplikasi CleanBro)

Katalog ini adalah acuan perilaku aplikasi SwiftUI di `apps/macos` dan parity
yang ditargetkan untuk Sapu Bersih. Rule macOS tidak dijalankan oleh engine
Rust Linux.

| Rule ID | Pola / aksi | Risiko | Mode |
| --- | --- | --- | --- |
| `macos.xcode.derived_data` | `~/Library/Developer/Xcode/DerivedData` | aman | trash |
| `macos.xcode.device_support` | `~/Library/Developer/Xcode/iOS DeviceSupport` | aman | trash |
| `macos.xcode.archives` | `~/Library/Developer/Xcode/Archives` | perlu tinjauan | trash |
| `macos.xcode.simulator_caches` | `~/Library/Developer/CoreSimulator/Caches` | aman | trash |
| `macos.cache.user` | `~/Library/Caches/**` | aman | trash |
| `macos.logs` | `~/Library/Logs/**` | aman | trash |
| `macos.crash_reports` | `~/Library/Logs/DiagnosticReports/**`, `~/Library/Application Support/CrashReporter/**` | aman | trash |
| `macos.incomplete_downloads` | `~/Downloads/*.download`, `~/Downloads/*.crdownload` | aman | trash |
| `macos.trash` | `~/.Trash/**` | perlu tinjauan | delete |
| `macos.homebrew.cleanup` | `brew cleanup` | aman | aksi CLI |
| `macos.uninstall.app_bundle` | bundle `.app` + support files terkait | perlu tinjauan | trash |

Guard macOS:

- `~/Library/Caches` tidak boleh mencakup bundle identifier aplikasi yang sedang
  berjalan bila cache-nya diperlukan saat runtime; eksekusi tetap lewat trash.
- `macos.uninstall.app_bundle` wajib menampilkan daftar support file sebelum
  eksekusi; aplikasi yang terpasang via Homebrew cask dihapus lewat `brew`.
- Deny-list macOS: `~/Library/Keychains/**`, `~/Library/Mobile Documents/**`,
  `~/Library/CloudStorage/**`, `~/Documents/**`, `~/Pictures/**`, `~/Movies/**`,
  `~/Music/**`, `~/.ssh/**`, `~/.gnupg/**`, `**/*.sqlite`, `**/*.db`.
- Analisis duplikat hanya menandai kandidat; penghapusan selalu manual per file.
