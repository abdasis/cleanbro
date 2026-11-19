/** Akses localStorage yang tahan penolakan browser (mode privat, dsb). */
export function readStorage(key: string): string | null {
  if (typeof window === "undefined") return null;

  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

export function writeStorage(key: string, value: string | null): void {
  if (typeof window === "undefined") return;

  try {
    if (value === null) window.localStorage.removeItem(key);
    else window.localStorage.setItem(key, value);
  } catch {
    // Penyimpanan bisa ditolak; preferensi tetap hidup di state sesi ini.
  }
}
