const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB", "PB"] as const;

/** Bahasa yang memengaruhi angka, tanggal, dan durasi (bukan satuan byte). */
export type FormatLanguage = "id" | "en";

const LOCALE: Record<FormatLanguage, string> = {
  id: "id-ID",
  en: "en-US",
};

const COUNT_FORMAT: Record<FormatLanguage, Intl.NumberFormat> = {
  id: new Intl.NumberFormat(LOCALE.id),
  en: new Intl.NumberFormat(LOCALE.en),
};

const DATE_FORMAT: Record<FormatLanguage, Intl.DateTimeFormat> = {
  id: new Intl.DateTimeFormat(LOCALE.id, {
    day: "2-digit",
    month: "short",
    year: "numeric",
  }),
  en: new Intl.DateTimeFormat(LOCALE.en, {
    day: "2-digit",
    month: "short",
    year: "numeric",
  }),
};

/**
 * Ukuran basis 1000, satu desimal untuk KB ke atas, pemisah titik.
 * Contoh: `34.1 GB`, `557 MB`, `0 B`. Tidak bergantung bahasa.
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

/** Jumlah file dengan pemisah ribuan sesuai bahasa, contoh `12.345` / `12,345`. */
export function formatCount(
  count: number,
  language: FormatLanguage = "id",
): string {
  if (!Number.isFinite(count)) return "0";
  return COUNT_FORMAT[language].format(Math.max(0, Math.round(count)));
}

/** Durasi singkat, contoh `12,4 detik` / `12.4 s`. */
export function formatDuration(
  durationMs: number,
  language: FormatLanguage = "id",
): string {
  if (!Number.isFinite(durationMs) || durationMs < 0) {
    return language === "en" ? "0 s" : "0 detik";
  }

  const value = new Intl.NumberFormat(LOCALE[language], {
    minimumFractionDigits: 1,
    maximumFractionDigits: 1,
  }).format(durationMs / 1000);

  return `${value} ${language === "en" ? "s" : "detik"}`;
}

/**
 * Tanggal lokal, contoh `19 Nov 2026, 14:32` (id) atau `Nov 19, 2026, 14:32`
 * (en). Menerima epoch milidetik; nilai di bawah 1e12 dianggap detik.
 */
export function formatDateTime(
  epoch: number,
  language: FormatLanguage = "id",
): string {
  if (!Number.isFinite(epoch) || epoch <= 0) return "-";

  const ms = epoch < 1e12 ? epoch * 1000 : epoch;
  const date = new Date(ms);
  if (Number.isNaN(date.getTime())) return "-";

  const time = `${String(date.getHours()).padStart(2, "0")}:${String(
    date.getMinutes(),
  ).padStart(2, "0")}`;

  return `${DATE_FORMAT[language].format(date)}, ${time}`;
}

/** Potong teks di tengah agar awal dan akhir path tetap terbaca. */
export function truncateMiddle(text: string, maxLength = 64): string {
  if (text.length <= maxLength) return text;
  const keep = Math.max(4, maxLength - 3);
  const head = Math.ceil(keep / 2);
  const tail = Math.floor(keep / 2);
  return `${text.slice(0, head)}...${text.slice(-tail)}`;
}
