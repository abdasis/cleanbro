import type { ReactElement } from "react";
import { Badge } from "../../components/ui/badge";
import { ProgressBar } from "../../components/ui/progress-bar";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { DiskSummary } from "../../lib/types";

type Warning = { tone: "warn" | "danger"; label: string };

export function DiskCard({ summary }: { summary: DiskSummary }): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

  let warning: Warning | null = null;
  if (summary.warn_level === "critical") {
    warning = { tone: "danger", label: t("disk.warnCritical") };
  } else if (summary.warn_level === "warn") {
    warning = { tone: "warn", label: t("disk.warnLow") };
  }

  return (
    <section
      aria-label={t("disk.aria")}
      className="rounded-cb border border-line bg-surface p-6"
    >
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0">
          <p className="text-ink-muted">
            {t("disk.mount", { mount: summary.mount })}
          </p>
          <p className="mt-1 text-2xl font-semibold tracking-[-0.02em] tabular-nums">
            {format.bytes(summary.used)}
            <span className="ml-2 text-[13px] font-normal text-ink-muted">
              {t("disk.of", { size: format.bytes(summary.total) })}
            </span>
          </p>
        </div>
        <div className="shrink-0 text-right">
          <p className="text-2xl font-semibold tracking-[-0.02em] tabular-nums">
            {format.percent(summary.percent)}
          </p>
          <p className="text-ink-muted">{t("disk.used")}</p>
        </div>
      </div>

      <ProgressBar
        value={summary.percent}
        label={t("disk.progress")}
        className="mt-4"
      />

      <div className="mt-2 flex items-center justify-between gap-3">
        <span className="text-ink-muted tabular-nums">
          {t("disk.free", { size: format.bytes(summary.free) })}
        </span>
        {warning ? <Badge tone={warning.tone}>{warning.label}</Badge> : null}
      </div>
    </section>
  );
}
