import { describe, expect, test } from "bun:test";
import {
  formatBytes,
  formatCount,
  formatDateTime,
  formatDuration,
  formatPercent,
  truncateMiddle,
} from "./format";

describe("formatBytes", () => {
  test("nol byte", () => {
    expect(formatBytes(0)).toBe("0 B");
  });

  test("di bawah 1 MB tetap memakai satuan kecil", () => {
    expect(formatBytes(999)).toBe("999 B");
    expect(formatBytes(557_000)).toBe("557 KB");
  });

  test("megabyte tanpa desimal nol", () => {
    expect(formatBytes(557_000_000)).toBe("557 MB");
  });

  test("gigabyte satu desimal", () => {
    expect(formatBytes(34_100_000_000)).toBe("34.1 GB");
  });

  test("nilai tidak valid", () => {
    expect(formatBytes(Number.NaN)).toBe("0 B");
    expect(formatBytes(-5)).toBe("0 B");
  });
});

describe("formatPercent", () => {
  test("persen bulat", () => {
    expect(formatPercent(85)).toBe("85%");
    expect(formatPercent(85.4)).toBe("85%");
  });

  test("dibatasi 0-100", () => {
    expect(formatPercent(120)).toBe("100%");
    expect(formatPercent(-3)).toBe("0%");
  });
});

describe("format tambahan", () => {
  test("jumlah file id-ID", () => {
    expect(formatCount(12345)).toBe("12.345");
  });

  test("durasi satu desimal koma", () => {
    expect(formatDuration(12_400)).toBe("12,4 detik");
  });

  test("tanggal lokal dengan titik dua", () => {
    const ts = new Date(2026, 10, 19, 14, 32).getTime();
    expect(formatDateTime(ts)).toBe("19 Nov 2026, 14:32");
    expect(formatDateTime(ts / 1000)).toBe("19 Nov 2026, 14:32");
    expect(formatDateTime(0)).toBe("-");
  });

  test("path panjang dipotong di tengah", () => {
    expect(truncateMiddle("pendek", 20)).toBe("pendek");
    const result = truncateMiddle("/home/user/proyek/node_modules/berat", 20);
    expect(result).toHaveLength(20);
    expect(result.startsWith("/home/use")).toBe(true);
    expect(result.endsWith("berat")).toBe(true);
    expect(result).toContain("...");
  });
});

describe("format mengikuti bahasa", () => {
  test("jumlah ribuan en-US", () => {
    expect(formatCount(12345, "en")).toBe("12,345");
    expect(formatCount(12345, "id")).toBe("12.345");
  });

  test("durasi en memakai titik dan satuan s", () => {
    expect(formatDuration(12_400, "en")).toBe("12.4 s");
    expect(formatDuration(0, "en")).toBe("0.0 s");
  });

  test("tanggal en-US", () => {
    const ts = new Date(2026, 10, 19, 14, 32).getTime();
    expect(formatDateTime(ts, "en")).toBe("Nov 19, 2026, 14:32");
  });
});
