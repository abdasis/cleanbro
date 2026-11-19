# Arsitektur Teknis: Sapu Bersih (Linux)

Versi: 1.1
Target: Tauri 2 (stabil), Rust 1.99 (edition 2021), React 19, TypeScript 5.x, Vite 7,
Tailwind CSS 4, shadcn/ui, bun 1.4

Dokumen ini berlaku untuk `apps/linux`. Aplikasi macOS (`apps/macos`) adalah
aplikasi SwiftUI yang sudah ada dan tidak memakai arsitektur ini; lihat
[PLATFORMS.md](PLATFORMS.md).

## 0. Layout monorepo

```text
Cargo.toml             # cargo workspace: crates/cleanbro-core, apps/linux/src-tauri
package.json           # bun workspaces: apps/*, packages/*
crates/cleanbro-core/           # tipe + engine bersama, tanpa tipe Tauri
  src/{lib,error,models,rules}.rs
packages/ui/           # AppShell React bersama
apps/linux/            # Sapu Bersih (Tauri)
  src/                 # React
  src-tauri/           # Rust shell + command
apps/macos/            # CleanBro (SwiftUI existing)
docs/                  # spesifikasi, rule, UX, task
```

Aturan: `crates/cleanbro-core` tidak boleh bergantung pada `tauri`; command Tauri tinggal
di `apps/linux/src-tauri`; UI bersama di `packages/ui`.

## 1. Model proses

```text
+------------------------------ WebView (React 19) ------------------------------+
|  Layer UI (shadcn/ui, Lucide, Geist)                                           |
|  state: TanStack Query untuk data scan + Zustand untuk pilihan item            |
+------------------------------- invoke / listen --------------------------------+
|                            Tauri Core (Rust)                                   |
|  commands/*.rs      -> thin, hanya validasi input + delegasi                   |
|  services/*.rs      -> orkestrasi scan, preview, execute, history              |
|  rules/*.rs         -> catalog rule, guard, klasifikasi                        |
|  engine/*.rs        -> walker paralel, agregasi ukuran, cancel token           |
|  adapters/*.rs      -> docker, apt, journald, trash                            |
|  store/*.rs         -> SQLite (riwayat, audit, undo journal)                   |
+--------------------------------------------------------------------------------+
```

Aturan pembagian: command tidak boleh berisi logika bisnis, service tidak boleh
tahu tipe Tauri, engine tidak boleh tahu SQL, dan rule tidak boleh melakukan I/O
selain stat.

## 2. Struktur direktori

```text
apps/linux/src-tauri/src/
  main.rs              # bootstrap tauri::Builder, plugin, state
  lib.rs               # expose run(), daftar command
  error.rs             # AppError + impl Serialize (kode error stabil)
  commands/
    scan.rs            # start_scan, cancel_scan
    preview.rs         # preview_selection, set_item_excluded
    execute.rs         # execute_clean
    history.rs         # list_history, undo_action
    system.rs          # disk_summary, app_info
  services/
    scan_service.rs    # orkestrasi scan + emit progress
    plan_service.rs    # bangun CleanPlan dari findings + pilihan user
    exec_service.rs    # eksekusi plan, dua fase validasi
  engine/
    walker.rs          # jwalk + pruning + depth limit
    size_map.rs        # agregasi ukuran per kategori
    cancel.rs          # CancellationToken wrapper
    big_files.rs       # heuristik file besar
  rules/
    catalog.rs         # daftar Rule static (lihat RULES.md)
    matcher.rs         # pencocokan path -> rule
    guard.rs           # deny-list + TOCTOU revalidation
    risk.rs            # RiskLevel + alasan manusiawi
  adapters/
    docker.rs          # baca df/ps + prune via CLI
    system_cache.rs    # apt, journald via pkexec (opsional)
    trash.rs           # trash::delete + restore
  store/
    db.rs              # koneksi, migrasi
    scan_repo.rs       # simpan scan + findings
    action_repo.rs     # simpan aksi + item + undo journal
  models/              # struct serde untuk IPC (dipakai juga oleh typegen)

apps/linux/src/
  main.tsx
  app/
    router.tsx
    layout/{app-shell,sidebar,titlebar}.tsx
  features/
    overview/{disk-card,trend-chart}.tsx
    scan/{scan-button,scan-progress,category-list}.tsx
    preview/{preview-sheet,item-table,size-summary}.tsx
    execute/{execute-dialog,progress-list,result-summary}.tsx
    history/{history-table,undo-button}.tsx
    settings/{language-toggle,rules-toggle}.tsx
  components/ui/       # shadcn/ui hasil generate
  lib/{ipc.ts,format.ts,query.ts,i18n.ts}
  hooks/{use-scan.ts,use-execute.ts,use-history.ts}
```

Batas 300 baris per file berlaku mutlak. `category-list.tsx` yang membengkak
dipecah menjadi `category-row.tsx` dan `category-detail.tsx`.

## 3. Kontrak IPC

Semua command mengembalikan `Result<T, AppError>`. `AppError` berbentuk:

```rust
pub struct AppError { pub code: ErrorCode, pub message: String, pub detail: Option<String> }
pub enum ErrorCode { NotFound, PermissionDenied, InvalidPath, GuardRejected,
                     Busy, Cancelled, Io, Internal }
```

Command publik (nama stabil, jadi kontrak API internal):

| Command | Input | Output |
| --- | --- | --- |
| `disk_summary` | `{}` | `DiskSummary { total, used, free, percent, mount, warn_level }` |
| `start_scan` | `{ mode: "quick" \| "deep", categories?: RuleId[] }` | `{ scan_id: String }` |
| `cancel_scan` | `{ scan_id }` | `{ cancelled: bool }` |
| `preview_selection` | `{ scan_id, rule_ids: RuleId[] }` | `CleanPlan` |
| `set_item_excluded` | `{ scan_id, item_id, excluded: bool }` | `CleanPlan` |
| `execute_clean` | `{ scan_id, item_ids: String[], mode: "trash" \| "delete", confirm_token: String }` | `{ action_id: String }` |
| `list_history` | `{ limit, offset }` | `{ items: ActionSummary[], total }` |
| `undo_action` | `{ action_id }` | `{ restored, failed }` |
| `app_info` | `{}` | `{ version, platform, rules_version }` |

Event (dari Rust ke UI, memakai `emit`):

| Event | Payload | Kapan |
| --- | --- | --- |
| `scan://progress` | `{ scan_id, scanned_files, current_path, categories_done }` | tiap 250 ms (throttled) |
| `scan://category` | `{ scan_id, rule_id, bytes, item_count }` | saat satu kategori selesai |
| `scan://finished` | `{ scan_id, partial, duration_ms }` | akhir scan |
| `clean://progress` | `{ action_id, item_id, state, freed_bytes }` | per item |
| `clean://finished` | `{ action_id, deleted, failed, skipped, freed_bytes }` | akhir aksi |

Throttling wajib: emit maksimal 4 kali per detik per event type agar WebView
tidak kelaparan.

## 4. Engine scan

- Walker: `jwalk` (paralel, berbasis rayon) dengan `max_depth` per rule.
- Pruning: direktori sistem (`/proc`, `/sys`, `/dev`, `/run`) langsung dilewati;
  `node_modules` di dalam kategori artefak tidak ditelusuri lebih dalam,
  ukurannya diambil dari hasil walk tingkat atas.
- Ukuran: jumlah `metadata.len()` per file; blok disk diambil dari
  `st_blocks * 512` bila tersedia agar estimasi sama dengan `du`.
- Batas: `--max-files` 5 juta, `--deadline` 120 detik, keduanya mengembalikan
  hasil parsial dengan flag.
- Cancellation: satu `CancellationToken` per scan; walker memeriksa tiap 4096
  entri dan berhenti dengan `ErrorCode::Cancelled`.
- Identifikasi proyek: naik dari direktori artefak sampai menemukan penanda
  root; nama proyek = nama direktori root yang dipakai sebagai label UI.

## 5. Engine rule dan safety

Dua lapis, keduanya harus lolos:

1. **Matcher** (allowlist): path harus cocok dengan `Rule` yang terdaftar.
   Pencocokan memakai `globset` terhadap path absolut yang dinormalisasi.
2. **Guard** (deny-list + prasyarat): menolak bila path berada di dalam
   deny-list, bukan milik pengguna, merupakan mount point berbeda, atau
   prasyarat rule tidak terpenuhi (misal: repo git tidak boleh kotor).

Eksekusi mengulang kedua lapis tepat sebelum menghapus (TOCTOU). Path yang
gagal validasi ulang dicatat `skipped` dengan alasan.

Penghapusan lewat `trash::delete` untuk mode `trash`; `std::fs::remove_dir_all`
untuk mode `delete` setelah konfirmasi token `HAPUS`. Token dibuat server-side
dari `(scan_id, item_ids, action_hash)` dan kedaluwarsa dalam 5 menit.

Audit log: setiap keputusan guard ditulis ke tabel `audit` dengan
`(ts, scan_id, item_id, rule_id, decision, reason)`.

## 6. Adapter docker

```text
docker system df --format '{{json .}}'   -> ruang per tipe
docker ps -a --format '{{json .}}'       -> container + status
docker image ls --format '{{json .}}'    -> image + referensi
docker volume ls --format '{{json .}}'   -> volume
```

Adapter hanya membaca. Aksi prune dijalankan sebagai perintah eksplisit:
`docker container prune -f`, `docker image prune -f` (tanpa `-a` di v0.1),
`docker volume prune -f` hanya untuk volume tanpa container. Volume yang
terpasang ke container hidup ditandai `in_use` dan tidak dapat dipilih.

Bila binary docker tidak ada, kategori docker berstatus `unavailable`, bukan
error global.

## 7. Penyimpanan

SQLite di `$APPDATA/sapu-bersih/state.db`, `rusqlite` dengan fitur `bundled`.

```sql
CREATE TABLE scans (id TEXT PRIMARY KEY, mode TEXT, started_at INTEGER,
                    finished_at INTEGER, partial INTEGER, bytes_found INTEGER);
CREATE TABLE findings (id TEXT PRIMARY KEY, scan_id TEXT, rule_id TEXT,
                       path TEXT, bytes INTEGER, item_count INTEGER,
                       risk TEXT, project TEXT, excluded INTEGER DEFAULT 0);
CREATE TABLE actions (id TEXT PRIMARY KEY, scan_id TEXT, mode TEXT,
                      started_at INTEGER, finished_at INTEGER,
                      freed_bytes INTEGER, status TEXT, confirm_token TEXT);
CREATE TABLE action_items (id TEXT PRIMARY KEY, action_id TEXT, path TEXT,
                           bytes INTEGER, state TEXT, reason TEXT,
                           trash_ref TEXT);
CREATE TABLE audit (id INTEGER PRIMARY KEY AUTOINCREMENT, ts INTEGER,
                    scan_id TEXT, item_id TEXT, rule_id TEXT,
                    decision TEXT, reason TEXT);
CREATE TABLE undo_journal (action_id TEXT PRIMARY KEY, payload TEXT,
                           created_at INTEGER, expires_at INTEGER);
```

Retensi: `findings` dipangkas ke 20 scan terakhir; `audit` disimpan 90 hari.

## 8. Frontend

- Data server: TanStack Query, satu query key per scan (`['scan', id]`),
  invalidasi saat event `scan://finished`.
- State klien: Zustand untuk pilihan item dan filter kategori.
- IPC wrapper tunggal di `lib/ipc.ts`:

```ts
export async function invokeCommand<T>(cmd: CommandName, args?: object): Promise<T>
export function subscribe<T>(event: EventName, handler: (payload: T) => void): () => void
```

- Semua tipe dibangkitkan dari `models/` memakai `tauri-typegen` agar tidak ada
  duplikasi tipe manual. Bila typegen tidak dipakai, tipe ditulis di
  `apps/linux/src/lib/types.ts` dan diperiksa oleh `tsc --noEmit`.

## 9. Performa dan observability

- Scan berjalan di `tauri::async_runtime` dengan `spawn_blocking` untuk bagian
  CPU/IO berat.
- `tracing` + `tracing-subscriber` menulis ke `$APPDATA/logs/app.log` dengan
  rotasi 5 MB x 3.
- Metrik internal (durasi scan, jumlah file, byte, item gagal) ditulis ke tabel
  `actions` agar bisa dianalisis lokal tanpa telemetri jaringan.
- Profiling: `cargo flamegraph` untuk scan; React DevTools untuk render list
  panjang (wajib virtualisasi bila item > 200).

## 10. Build, CI, dan distribusi

- `bun run build:linux` (root) menjalankan `tauri build` di `apps/linux` dan
  menghasilkan `.deb` + AppImage.
- `apps/macos/build.sh` tetap membangun `CleanBro.app` universal via SwiftPM.
- GitHub Actions:
  - `linux.yml`: `bun install`, `bun run typecheck`, `cargo fmt --check`,
    `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`.
  - `build.yml` (lama, tetap): Swift app dan CLI Go macOS.
- Signing: Linux tanpa signing; macOS ad-hoc seperti sebelumnya.
- Ukuran installer Linux dijaga di bawah 25 MB dengan `strip = true`,
  `lto = "thin"`, dan `opt-level = "s"` pada profil release.

## 11. Keputusan desain dan alasannya

| Keputusan | Alternatif | Alasan |
| --- | --- | --- |
| Rust native walker (`jwalk`) | Shell ke `du` | Parsing output rapuh, tidak bisa cancel, tidak portabel |
| SQLite lokal | `tauri-plugin-store` JSON | Butuh query riwayat, agregasi, dan audit terstruktur |
| Trash sebagai default | Hapus permanen | Undo adalah syarat utama keamanan produk |
| Adapter CLI untuk docker | Docker API socket | Socket butuh akses root, CLI cukup dan sudah ada |
| Allowlist rule | Blocklist folder | Blocklist selalu kalah cepat dibanding jumlah pola berbahaya |
| Tanpa telemetri | Analytics | Aplikasi offline, data pengguna sensitif |
