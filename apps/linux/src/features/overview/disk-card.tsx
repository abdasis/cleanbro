import type { ReactElement } from "react";
import { Badge } from "../../components/ui/badge";
import { ProgressBar } from "../../components/ui/progress-bar";
import { formatBytes, formatPercent } from "../../lib/format";
import type { DiskSummary, WarnLevel } from "../../lib/types";

type Warning = { tone: "warn" | "danger"; label: string };

function warningFor(level: WarnLevel): Warning | null {
  if (level === "critical") {
    return { tone: "danger", label: "Sisa disk kritis" };
  }
  if (level === "warn") {
    return { tone: "warn", label: "Sisa disk rendah" };
  }
  return null;
}

export function DiskCard({ summary }: { summary: DiskSummary }): ReactElement {
  const warning = warningFor(summary.warn_level);

  return (
    <section
      aria-label="Ringkasan kapasitas disk"
      className="rounded-cb border border-line bg-surface p-6"
    >
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0">
          <p className="text-ink-muted">Disk {summary.mount}</p>
          <p className="mt-1 text-2xl font-semibold tracking-[-0.02em] tabular-nums">
            {formatBytes(summary.used)}
            <span className="ml-2 text-[13px] font-normal text-ink-muted">
              dari {formatBytes(summary.total)}
            </span>
          </p>
        </div>
        <div className="shrink-0 text-right">
          <p className="text-2xl font-semibold tracking-[-0.02em] tabular-nums">
            {formatPercent(summary.percent)}
          </p>
          <p className="text-ink-muted">terpakai</p>
        </div>
      </div>

      <ProgressBar
        value={summary.percent}
        label="Pemakaian disk"
        className="mt-4"
      />

      <div className="mt-2 flex items-center justify-between gap-3">
        <span className="text-ink-muted tabular-nums">
          Sisa {formatBytes(summary.free)}
        </span>
        {warning ? <Badge tone={warning.tone}>{warning.label}</Badge> : null}
      </div>
    </section>
  );
}
