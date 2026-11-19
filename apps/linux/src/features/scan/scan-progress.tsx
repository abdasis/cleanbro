import type { ReactElement } from "react";
import { Badge } from "../../components/ui/badge";
import { Button } from "../../components/ui/button";
import { ProgressBar } from "../../components/ui/progress-bar";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { ScanFinished, ScanProgress } from "../../lib/types";

export type ScanProgressProps = {
  progress: ScanProgress | null;
  finished: ScanFinished | null;
  categoriesDone: number;
  pending: boolean;
  onCancel: () => void;
};

export function ScanProgressPanel({
  progress,
  finished,
  categoriesDone,
  pending,
  onCancel,
}: ScanProgressProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const running = finished === null;
  const path = progress?.current_path ?? t("scan.preparing");

  return (
    <section
      aria-labelledby="scan-progress-heading"
      className="rounded-cb border border-line bg-surface p-6"
    >
      <div className="flex items-center justify-between gap-4">
        <div className="flex min-w-0 items-center gap-2">
          <h3 id="scan-progress-heading" className="font-semibold">
            {running ? t("scan.running") : t("scan.finished")}
          </h3>
          {finished?.partial ? (
            <Badge tone="warn">{t("scan.partial")}</Badge>
          ) : null}
        </div>
        {running ? (
          <Button variant="outline" onClick={onCancel} disabled={pending}>
            {t("scan.cancel")}
          </Button>
        ) : null}
      </div>

      <ProgressBar
        value={finished ? 100 : undefined}
        label={t("scan.progress")}
        className="mt-4"
      />

      <div className="mt-2 flex flex-wrap items-center gap-x-2 text-ink-muted tabular-nums">
        <span>
          {t("scan.files", {
            count: format.count(progress?.scanned_files ?? 0),
          })}
        </span>
        <span aria-hidden="true">/</span>
        <span>
          {t("scan.categoriesDone", {
            count: format.count(categoriesDone),
          })}
        </span>
        {finished ? (
          <>
            <span aria-hidden="true">/</span>
            <span>{format.duration(finished.duration_ms)}</span>
          </>
        ) : null}
      </div>

      {running ? (
        <p
          title={path}
          className="cb-path-tail mt-2 truncate font-mono text-[12px] text-ink-muted"
        >
          {path}
        </p>
      ) : null}
    </section>
  );
}
