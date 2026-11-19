import { useState, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { Dialog } from "../../components/ui/dialog";
import { ErrorState } from "../../components/ui/error-state";
import { useHistory, useUndoAction } from "../../hooks/use-history";
import { formatBytes, formatCount } from "../../lib/format";
import type { ActionSummary } from "../../lib/types";
import { HistoryTable } from "./history-table";

const STATUS_FILTERS = [
  { value: "all", label: "Semua status" },
  { value: "done", label: "Selesai" },
  { value: "partial", label: "Sebagian" },
  { value: "failed", label: "Gagal" },
] as const;

type StatusFilter = (typeof STATUS_FILTERS)[number]["value"];

export function HistoryPage(): ReactElement {
  const history = useHistory();
  const undo = useUndoAction();
  const [status, setStatus] = useState<StatusFilter>("all");
  const [pendingUndo, setPendingUndo] = useState<ActionSummary | null>(null);

  const visible =
    status === "all"
      ? history.items
      : history.items.filter((action) => action.status === status);

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
          <h2 className="font-semibold">Riwayat pembersihan</h2>
          <p className="text-ink-muted tabular-nums">
            {formatCount(history.total)} aksi tercatat,{" "}
            {formatBytes(history.items.reduce((sum, item) => sum + item.freed_bytes, 0))}{" "}
            dibebaskan pada data yang dimuat.
          </p>
        </div>

        <label className="flex items-center gap-2 text-ink-muted">
          <span className="sr-only">Filter status</span>
          <select
            aria-label="Filter status"
            value={status}
            onChange={(event) => setStatus(event.target.value as StatusFilter)}
            className="h-8 rounded-cb border border-line bg-surface px-2 text-ink"
          >
            {STATUS_FILTERS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
      </div>

      {undo.isError ? (
        <p className="rounded-cb border border-line px-4 py-3 text-danger">
          Undo gagal diproses. Coba lagi.
        </p>
      ) : null}

      {history.isPending ? (
        <p className="rounded-cb border border-line bg-surface px-6 py-8 text-ink-muted">
          Memuat riwayat...
        </p>
      ) : history.isError ? (
        <ErrorState
          title="Riwayat tidak bisa dimuat"
          error={history.error}
          onRetry={history.refetch}
        />
      ) : history.items.length === 0 ? (
        <div className="rounded-cb border border-line bg-surface px-6 py-8">
          <p className="font-semibold">Belum ada aksi pembersihan</p>
          <p className="mt-1 text-ink-muted">
            Aksi yang kamu jalankan akan tercatat di sini.
          </p>
        </div>
      ) : visible.length === 0 ? (
        <div className="rounded-cb border border-line bg-surface px-6 py-8">
          <p className="font-semibold">Tidak ada aksi dengan status ini</p>
          <p className="mt-1 text-ink-muted">
            Coba pilih status lain atau muat data lebih banyak.
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
            {history.isFetchingNextPage ? "Memuat..." : "Muat lagi"}
          </Button>
        </div>
      ) : null}

      <Dialog
        open={pendingUndo !== null}
        title="Pulihkan item dari trash?"
        description={
          pendingUndo
            ? `${formatCount(pendingUndo.item_count)} item (${formatBytes(
                pendingUndo.freed_bytes,
              )}) akan dikembalikan ke path asal.`
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
              Batal
            </Button>
            <Button
              variant="primary"
              disabled={undo.isPending}
              onClick={confirmUndo}
            >
              {undo.isPending ? "Memulihkan..." : "Pulihkan"}
            </Button>
          </>
        }
      >
        <p className="text-ink-muted">
          Item yang sudah tidak ada di trash akan dilaporkan gagal dan tidak
          dipulihkan.
        </p>
      </Dialog>
    </div>
  );
}
