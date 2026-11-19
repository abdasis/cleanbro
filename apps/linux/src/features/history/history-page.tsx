import { useState, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { Dialog } from "../../components/ui/dialog";
import { ErrorState } from "../../components/ui/error-state";
import { useHistory, useUndoAction } from "../../hooks/use-history";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { TranslationKey } from "../../lib/i18n/dictionary";
import type { ActionSummary } from "../../lib/types";
import { HistoryTable } from "./history-table";

const STATUS_FILTERS: Array<{ value: string; labelKey: TranslationKey }> = [
  { value: "all", labelKey: "status.all" },
  { value: "done", labelKey: "status.done" },
  { value: "partial", labelKey: "status.partial" },
  { value: "failed", labelKey: "status.failed" },
];

type StatusFilter = (typeof STATUS_FILTERS)[number]["value"];

export function HistoryPage(): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const history = useHistory();
  const undo = useUndoAction();
  const [status, setStatus] = useState<StatusFilter>("all");
  const [pendingUndo, setPendingUndo] = useState<ActionSummary | null>(null);

  const visible =
    status === "all"
      ? history.items
      : history.items.filter((action) => action.status === status);

  const freedLoaded = history.items.reduce(
    (sum, item) => sum + item.freed_bytes,
    0,
  );

  const confirmUndo = () => {
    if (!pendingUndo) return;
    undo.mutate(pendingUndo.id, {
      onSuccess: () => setPendingUndo(null),
    });
  };

  return (
    <div className="mx-auto flex w-full max-w-5xl flex-col gap-4">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h2 className="font-semibold">{t("history.title")}</h2>
          <p className="text-ink-muted tabular-nums">
            {t("history.summary", {
              actions: format.count(history.total),
              size: format.bytes(freedLoaded),
            })}
          </p>
        </div>

        <label className="flex items-center gap-2 text-ink-muted">
          <span className="sr-only">{t("history.filter")}</span>
          <select
            aria-label={t("history.filter")}
            value={status}
            onChange={(event) => setStatus(event.target.value as StatusFilter)}
            className="h-8 rounded-cb border border-line bg-surface px-2 text-ink"
          >
            {STATUS_FILTERS.map((option) => (
              <option key={option.value} value={option.value}>
                {t(option.labelKey)}
              </option>
            ))}
          </select>
        </label>
      </div>

      {undo.isError ? (
        <p className="rounded-cb border border-line px-4 py-3 text-danger">
          {t("history.undoError")}
        </p>
      ) : null}

      {history.isPending ? (
        <p className="rounded-cb border border-line bg-surface px-6 py-8 text-ink-muted">
          {t("history.loading")}
        </p>
      ) : history.isError ? (
        <ErrorState
          title={t("history.errorTitle")}
          error={history.error}
          onRetry={history.refetch}
        />
      ) : history.items.length === 0 ? (
        <div className="rounded-cb border border-line bg-surface px-6 py-8">
          <p className="font-semibold">{t("history.emptyTitle")}</p>
          <p className="mt-1 text-ink-muted">{t("history.emptyHint")}</p>
        </div>
      ) : visible.length === 0 ? (
        <div className="rounded-cb border border-line bg-surface px-6 py-8">
          <p className="font-semibold">{t("history.filterEmptyTitle")}</p>
          <p className="mt-1 text-ink-muted">
            {t("history.filterEmptyHint")}
          </p>
        </div>
      ) : (
        <HistoryTable items={visible} onUndo={setPendingUndo} />
      )}

      {history.hasMore ? (
        <div className="flex justify-center">
          <Button
            variant="outline"
            disabled={history.isFetchingNextPage}
            onClick={history.loadMore}
          >
            {history.isFetchingNextPage
              ? t("history.loadingMore")
              : t("history.loadMore")}
          </Button>
        </div>
      ) : null}

      <Dialog
        open={pendingUndo !== null}
        title={t("history.undoTitle")}
        description={
          pendingUndo
            ? t("history.undoDesc", {
                count: format.count(pendingUndo.item_count),
                size: format.bytes(pendingUndo.freed_bytes),
              })
            : undefined
        }
        size="sm"
        dismissible={!undo.isPending}
        onClose={() => setPendingUndo(null)}
        footer={
          <>
            <Button
              variant="outline"
              disabled={undo.isPending}
              onClick={() => setPendingUndo(null)}
            >
              {t("common.cancel")}
            </Button>
            <Button
              variant="primary"
              disabled={undo.isPending}
              onClick={confirmUndo}
            >
              {undo.isPending ? t("history.undoing") : t("history.undo")}
            </Button>
          </>
        }
      >
        <p className="text-ink-muted">{t("history.undoHint")}</p>
      </Dialog>
    </div>
  );
}
