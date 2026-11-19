const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB", "PB"] as const;

/**
 * Ukuran basis 1000, satu desimal untuk KB ke atas, pemisah titik.
 * Contoh: `34.1 GB`, `557 MB`, `0 B`.
 */
export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes <= 0) return "0 B";

  const exponent = Math.min(
    Math.floor(Math.log(bytes) / Math.log(1000)),
    BYTE_UNITS.length - 1,
  );
  const value = bytes / 1000 ** exponent;
  const unit = BYTE_UNITS[exponent] ?? "B";

  if (exponent === 0) return `${Math.round(value)} ${unit}`;

  return `${value.toFixed(1).replace(/\.0$/, "")} ${unit}`;
}

/** Persen bulat 0-100, contoh `85%`. */
export function formatPercent(percent: number): string {
  if (!Number.isFinite(percent)) return "0%";
  return `${Math.round(Math.min(100, Math.max(0, percent)))}%`;
}

/** Jumlah file dengan pemisah ribuan id-ID, contoh `12.345`. */
export function formatCount(count: number): string {
  if (!Number.isFinite(count)) return "0";
  return new Intl.NumberFormat("id-ID").format(Math.max(0, Math.round(count)));
}

/** Durasi singkat untuk ringkasan scan, contoh `12,4 detik`. */
export function formatDuration(durationMs: number): string {
  if (!Number.isFinite(durationMs) || durationMs < 0) return "0 detik";
  const seconds = durationMs / 1000;
  return `${seconds.toFixed(1).replace(".", ",")} detik`;
}

const DATE_FORMAT = new Intl.DateTimeFormat("id-ID", {
  day: "2-digit",
  month: "short",
  year: "numeric",
});

/**
 * Tanggal lokal, contoh `19 Nov 2026, 14:32`. Menerima epoch milidetik;
 * nilai di bawah 1e12 dianggap detik (backend lama memakai detik).
 */
export function formatDateTime(epoch: number): string {
  if (!Number.isFinite(epoch) || epoch <= 0) return "-";

  const ms = epoch < 1e12 ? epoch * 1000 : epoch;
  const date = new Date(ms);
  if (Number.isNaN(date.getTime())) return "-";

  const time = `${String(date.getHours()).padStart(2, "0")}:${String(
    date.getMinutes(),
  ).padStart(2, "0")}`;

  return `${DATE_FORMAT.format(date)}, ${time}`;
}

/** Potong teks di tengah agar awal dan akhir path tetap terbaca. */
export function truncateMiddle(text: string, maxLength = 64): string {
  if (text.length <= maxLength) return text;
  const keep = Math.max(4, maxLength - 3);
  const head = Math.ceil(keep / 2);
  const tail = Math.floor(keep / 2);
  return `${text.slice(0, head)}...${text.slice(-tail)}`;
}
