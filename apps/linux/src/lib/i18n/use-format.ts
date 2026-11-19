import { useMemo } from "react";
import {
  formatBytes,
  formatCount,
  formatDateTime,
  formatDuration,
  formatPercent,
  truncateMiddle,
} from "../format";
import { useI18n } from "./i18n-context";

export type Format = {
  bytes: (bytes: number) => string;
  percent: (percent: number) => string;
  count: (count: number) => string;
  duration: (durationMs: number) => string;
  dateTime: (epoch: number) => string;
  truncateMiddle: (text: string, maxLength?: number) => string;
};

/** Formatter yang mengikuti bahasa aktif. */
export function useFormat(): Format {
  const { language } = useI18n();

  return useMemo(
    () => ({
      bytes: formatBytes,
      percent: formatPercent,
      count: (count: number) => formatCount(count, language),
      duration: (durationMs: number) => formatDuration(durationMs, language),
      dateTime: (epoch: number) => formatDateTime(epoch, language),
      truncateMiddle,
    }),
    [language],
  );
}
