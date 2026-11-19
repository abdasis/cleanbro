import type { ReactElement } from "react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { Button } from "../../components/ui/button";
import { formatBytes, formatCount, formatDateTime } from "../../lib/format";
import type { ActionSummary, DeleteMode } from "../../lib/types";

const STATUS_LABEL: Record<string, string> = {
  done: "Selesai",
  partial: "Sebagian",
  failed: "Gagal",
  running: "Berjalan",
  undone: "Dipulihkan",
};

const STATUS_TONE: Record<string, BadgeTone> = {
  failed: "danger",
  partial: "warn",
};

const MODE_LABEL: Record<DeleteMode, string> = {
  trash: "Ke trash",
  delete: "Hapus permanen",
};

export type HistoryTableProps = {
  items: ActionSummary[];
  onUndo: (action: ActionSummary) => void;
};

export function HistoryTable({
  items,
  onUndo,
}: HistoryTableProps): ReactElement {
  return (
    <div className="overflow-x-auto rounded-cb border border-line bg-surface">
      <table className="w-full min-w-[560px]">
        <caption className="sr-only">Riwayat aksi pembersihan</caption>
        <thead>
          <tr className="border-b border-line text-left text-[11px] uppercase tracking-[0.06em] text-ink-muted">
            <th scope="col" className="px-4 py-2 font-medium">
              Waktu
            </th>
            <th scope="col" className="px-3 py-2 font-medium">
              Mode
            </th>
            <th scope="col" className="px-3 py-2 text-right font-medium">
              Item
            </th>
            <th scope="col" className="px-3 py-2 text-right font-medium">
              Dibebaskan
            </th>
            <th scope="col" className="px-3 py-2 font-medium">
              Status
            </th>
            <th scope="col" className="px-4 py-2 text-right font-medium">
              Aksi
            </th>
          </tr>
        </thead>
        <tbody className="divide-y divide-line">
          {items.map((action) => (
            <tr key={action.id} className="h-11">
              <td className="whitespace-nowrap px-4 py-2 font-mono text-[12px] tabular-nums">
                {formatDateTime(action.started_at)}
              </td>
              <td className="px-3 py-2">
                {MODE_LABEL[action.mode] ?? action.mode}
              </td>
              <td className="px-3 py-2 text-right tabular-nums">
                {formatCount(action.item_count)}
              </td>
              <td className="px-3 py-2 text-right font-mono text-[12px] tabular-nums">
                {formatBytes(action.freed_bytes)}
              </td>
              <td className="px-3 py-2">
                <Badge tone={STATUS_TONE[action.status] ?? "neutral"}>
                  {STATUS_LABEL[action.status] ?? action.status}
                </Badge>
              </td>
              <td className="px-4 py-2 text-right">
                <Button
                  variant="outline"
                  disabled={!action.undoable}
                  title={
                    action.undoable
                      ? "Pulihkan item dari trash"
                      : "Item sudah tidak ada di trash"
                  }
                  onClick={() => onUndo(action)}
                >
                  Undo
                </Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
