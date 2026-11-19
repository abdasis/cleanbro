import { useEffect, useState, type ReactElement } from "react";
import { ErrorState } from "../../components/ui/error-state";
import { useDiskSummary } from "../../hooks/use-disk-summary";
import { useScan } from "../../hooks/use-scan";
import { useEnabledRules } from "../../lib/enabled-rules";
import { useI18n } from "../../lib/i18n/i18n-context";
import { DockerSection } from "../docker/docker-section";
import { PreviewSheet } from "../preview/preview-sheet";
import { ScanActions } from "../scan/scan-actions";
import { ScanProgressPanel } from "../scan/scan-progress";
import { CategoryList } from "./category-list";
import { DiskCard } from "./disk-card";

export type OverviewPageProps = {
  onViewHistory: () => void;
};

export function OverviewPage({
  onViewHistory,
}: OverviewPageProps): ReactElement {
  const { t } = useI18n();
  const { enabled } = useEnabledRules();
  const disk = useDiskSummary();
  const scan = useScan(enabled);
  const [previewRule, setPreviewRule] = useState<string | null>(null);

  const scanning = scan.status === "running";
  const showScanPanel = scanning || scan.finished !== null;
  const noCategories = enabled !== null && enabled.length === 0;

  useEffect(() => {
    setPreviewRule(null);
  }, [scan.scanId]);

  const retryScan = () => {
    if (noCategories && !scanning) return;
    if (scanning) void scan.cancel();
    else void scan.start(scan.lastMode ?? "quick");
  };

  return (
    <div className="mx-auto flex w-full max-w-4xl flex-col gap-4">
      <section aria-labelledby="disk-section-heading">
        <div className="mb-2 flex items-center justify-between gap-3">
          <h2 id="disk-section-heading" className="font-semibold">
            {t("overview.diskSection")}
          </h2>
          {scanning ? null : (
            <ScanActions
              disabled={scan.pending || noCategories}
              onStart={(mode) => void scan.start(mode)}
            />
          )}
        </div>

        {noCategories ? (
          <p className="mb-2 text-ink-muted">{t("overview.noCategories")}</p>
        ) : null}

        {disk.isPending ? (
          <div className="rounded-cb border border-line bg-surface px-6 py-5 text-ink-muted">
            {t("overview.diskLoading")}
          </div>
        ) : disk.isError ? (
          <ErrorState
            title={t("overview.diskError")}
            error={disk.error}
            onRetry={() => void disk.refetch()}
          />
        ) : disk.data ? (
          <DiskCard summary={disk.data} />
        ) : null}
      </section>

      {scan.error ? (
        <ErrorState
          title={
            scanning
              ? t("overview.scanCancelError")
              : t("overview.scanStartError")
          }
          error={scan.error}
          onRetry={retryScan}
        />
      ) : null}

      {showScanPanel ? (
        <ScanProgressPanel
          progress={scan.progress}
          finished={scan.finished}
          categoriesDone={
            scan.progress?.categories_done ?? scan.categories.length
          }
          pending={scan.pending}
          onCancel={() => void scan.cancel()}
        />
      ) : null}

      <section aria-labelledby="category-section-heading">
        <div className="mb-2 flex items-center justify-between gap-3">
          <h2 id="category-section-heading" className="font-semibold">
            {t("overview.categories")}
          </h2>
          <p className="text-ink-muted">{t("overview.categoriesHint")}</p>
        </div>
        <CategoryList
          categories={scan.categories}
          scanning={scanning}
          onSelect={(category) => setPreviewRule(category.rule_id)}
        />
      </section>

      <DockerSection />

      {previewRule && scan.scanId ? (
        <PreviewSheet
          scanId={scan.scanId}
          ruleId={previewRule}
          onClose={() => setPreviewRule(null)}
          onViewHistory={() => {
            setPreviewRule(null);
            onViewHistory();
          }}
        />
      ) : null}
    </div>
  );
}
