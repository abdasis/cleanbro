import { useState, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { Dialog } from "../../components/ui/dialog";
import { useExecute } from "../../hooks/use-execute";
import { useUndoAction } from "../../hooks/use-history";
import { formatBytes } from "../../lib/format";
import type { DeleteMode, PlanItem } from "../../lib/types";
import { ProgressList } from "./progress-list";
import { ResultSummary } from "./result-summary";

const MODES: Array<{ value: DeleteMode; label: string }> = [
  { value: "trash", label: "Ke trash" },
  { value: "delete", label: "Hapus permanen" },
];

export type ExecuteDialogProps = {
  scanId: string;
  items: PlanItem[];
  requiresConfirmation: boolean;
  onClose: () => void;
  onFinish: () => void;
  onViewHistory: () => void;
};

export function ExecuteDialog({
  scanId,
  items,
  requiresConfirmation,
  onClose,
  onFinish,
  onViewHistory,
}: ExecuteDialogProps): ReactElement {
  const [mode, setMode] = useState<DeleteMode>("trash");
  const [confirmText, setConfirmText] = useState("");
  const undo = useUndoAction();

  const itemIds = items.map((item) => item.item_id);
  const execution = useExecute(scanId, itemIds);

  const totalBytes = items.reduce((sum, item) => sum + item.bytes, 0);
  const running = execution.phase === "running";
  const done = execution.phase === "done";
  const confirmed = !requiresConfirmation || confirmText === "HAPUS";
  const canStart = !running && confirmed && items.length > 0;
  const freedLive = Object.values(execution.items).reduce(
    (sum, item) => sum + (item.state === "done" ? item.freedBytes : 0),
    0,
  );

  const start = () => {
    if (canStart) void execution.execute(mode);
  };

  const footer = done ? (
    <>
      {mode === "trash" && execution.actionId ? (
        <Button
          variant="outline"
          disabled={undo.isPending || undo.isSuccess}
          onClick={() => undo.mutate(execution.actionId ?? "")}
        >
          {undo.isSuccess ? "Dipulihkan" : undo.isPending ? "Memulihkan..." : "Undo"}
        </Button>
      ) : null}
      <Button variant="outline" onClick={onViewHistory}>
        Lihat riwayat
      </Button>
      <Button variant="primary" onClick={onFinish}>
        Selesai
      </Button>
    </>
  ) : execution.phase === "error" ? (
    <Button variant="outline" onClick={onClose}>
      Tutup
    </Button>
  ) : running ? undefined : (
    <>
      <Button variant="outline" onClick={onClose}>
        Batal
      </Button>
      <Button variant="primary" disabled={!canStart} onClick={start}>
        Mulai membersihkan
      </Button>
    </>
  );

  return (
    <Dialog
      open
      title={done ? "Pembersihan selesai" : "Jalankan pembersihan"}
      description={
        done
          ? "Ringkasan hasil dan item yang perlu dicek."
          : `${items.length} item, ${formatBytes(totalBytes)} akan dibersihkan.`
      }
      size="lg"
      dismissible={!running}
      onClose={onClose}
      footer={running ? undefined : footer}
    >
      {running ? (
        <div className="flex flex-col gap-3">
          <div className="flex items-center justify-between gap-3">
            <p className="text-ink-muted">
              Sedang membersihkan {itemIds.length} item. Jangan tutup jendela
              ini.
            </p>
            <span
              aria-live="polite"
              className="shrink-0 font-mono text-[12px] tabular-nums"
            >
              {formatBytes(freedLive)} dibebaskan
            </span>
          </div>
          <ProgressList items={items} progress={execution.items} />
        </div>
      ) : done && execution.finished ? (
        <ResultSummary
          finished={execution.finished}
          durationMs={execution.durationMs}
          items={items}
          progress={execution.items}
        />
      ) : execution.phase === "error" ? (
        <div className="rounded-cb border border-line px-4 py-3">
          <p className="font-semibold">Pembersihan tidak bisa dijalankan</p>
          <p className="mt-1 text-ink-muted">
            {execution.error?.message ?? "Coba lagi sebentar."}
          </p>
          {execution.error ? (
            <code className="mt-2 inline-block font-mono text-[11px] text-ink-muted">
              {execution.error.code}
            </code>
          ) : null}
        </div>
      ) : (
        <div className="flex flex-col gap-4">
          <div>
            <p id="execute-mode-label" className="mb-1.5 text-ink-muted">
              Mode hapus
            </p>
            <div
              role="radiogroup"
              aria-labelledby="execute-mode-label"
              className="inline-flex rounded-cb border border-line p-0.5"
            >
              {MODES.map((option) => {
                const active = mode === option.value;
                const disabled = option.value === "delete";

                return (
                  <button
                    key={option.value}
                    type="button"
                    role="radio"
                    aria-checked={active}
                    disabled={disabled}
                    onClick={() => setMode(option.value)}
                    className={`h-8 rounded-[6px] px-3 text-[13px] transition-colors duration-150 ease-cb active:scale-[0.96] disabled:pointer-events-none disabled:opacity-40 ${
                      active
                        ? "bg-ink text-surface"
                        : "text-ink-muted hover:text-ink"
                    }`}
                  >
                    {option.label}
                  </button>
                );
              })}
            </div>
          </div>

          <p className="text-ink-muted">
            Hapus permanen belum tersedia karena belum ada kategori yang
            ditandai aman untuk dihapus permanen. Semua item masuk ke trash dan
            masih bisa dipulihkan.
          </p>

          {mode === "delete" ? (
            <p className="text-ink-muted">
              File dihapus permanen dan tidak bisa dipulihkan.
            </p>
          ) : null}

          {requiresConfirmation ? (
            <label className="flex flex-col gap-1.5">
              <span>
                Ketik <span className="font-mono">HAPUS</span> untuk
                melanjutkan
              </span>
              <input
                type="text"
                value={confirmText}
                onChange={(event) => setConfirmText(event.target.value)}
                autoComplete="off"
                spellCheck={false}
                aria-invalid={confirmText.length > 0 && !confirmed}
                className="h-8 w-48 rounded-cb border border-line bg-surface px-2 font-mono text-[13px] tracking-[0.1em] focus-visible:outline-2"
              />
              {confirmText.length > 0 && !confirmed ? (
                <span className="text-danger">
                  Teks belum sama persis dengan HAPUS.
                </span>
              ) : null}
            </label>
          ) : null}
        </div>
      )}
    </Dialog>
  );
}
