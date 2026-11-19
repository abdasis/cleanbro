import { useEffect, useState, type ReactElement } from "react";
import { ErrorState } from "../../components/ui/error-state";
import { useDiskSummary } from "../../hooks/use-disk-summary";
import { useScan } from "../../hooks/use-scan";
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
  const disk = useDiskSummary();
  const scan = useScan();
  const [previewRule, setPreviewRule] = useState<string | null>(null);

  const scanning = scan.status === "running";
  const showScanPanel = scanning || scan.finished !== null;

  useEffect(() => {
    setPreviewRule(null);
  }, [scan.scanId]);

  const retryScan = () => {
    if (scanning) void scan.cancel();
    else void scan.start(scan.lastMode ?? "quick");
  };

  return (
    <div className="mx-auto flex w-full max-w-4xl flex-col gap-4">
      <section aria-labelledby="disk-section-heading">
        <div className="mb-2 flex items-center justify-between gap-3">
          <h2 id="disk-section-heading" className="font-semibold">
            Ringkasan disk
          </h2>
          {scanning ? null : (
            <ScanActions
              disabled={scan.pending}
              onStart={(mode) => void scan.start(mode)}
            />
          )}
        </div>

        {disk.isPending ? (
          <div className="rounded-cb border border-line bg-surface px-6 py-5 text-ink-muted">
            Memuat ringkasan disk...
          </div>
        ) : disk.isError ? (
          <ErrorState
            title="Ringkasan disk tidak tersedia"
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
            scanning ? "Pembatalan gagal" : "Pemindaian belum bisa dimulai"
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
            Kategori
          </h2>
          <p className="text-ink-muted">Pilih kategori untuk melihat isinya</p>
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
