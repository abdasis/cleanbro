import type { ReactElement } from "react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { Button } from "../../components/ui/button";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { TranslationKey } from "../../lib/i18n/dictionary";
import type { ActionSummary, DeleteMode } from "../../lib/types";

const STATUS_KEY: Record<string, TranslationKey> = {
  done: "status.done",
  partial: "status.partial",
  failed: "status.failed",
  running: "status.running",
  undone: "status.undone",
};

const STATUS_TONE: Record<string, BadgeTone> = {
  failed: "danger",
  partial: "warn",
};

const MODE_KEY: Record<DeleteMode, TranslationKey> = {
  trash: "deleteMode.trash",
  delete: "deleteMode.permanent",
};

export type HistoryTableProps = {
  items: ActionSummary[];
  onUndo: (action: ActionSummary) => void;
};

export function HistoryTable({
  items,
  onUndo,
}: HistoryTableProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

  return (
    <div className="overflow-x-auto rounded-cb border border-line bg-surface">
      <table className="w-full min-w-[560px]">
        <caption className="sr-only">{t("history.tableCaption")}</caption>
        <thead>
          <tr className="border-b border-line text-left text-[11px] uppercase tracking-[0.06em] text-ink-muted">
            <th scope="col" className="px-4 py-2 font-medium">
              {t("table.time")}
            </th>
            <th scope="col" className="px-3 py-2 font-medium">
              {t("table.mode")}
            </th>
            <th scope="col" className="px-3 py-2 text-right font-medium">
              {t("table.item")}
            </th>
            <th scope="col" className="px-3 py-2 text-right font-medium">
              {t("table.freed")}
            </th>
            <th scope="col" className="px-3 py-2 font-medium">
              {t("table.status")}
            </th>
            <th scope="col" className="px-4 py-2 text-right font-medium">
              {t("table.action")}
            </th>
          </tr>
        </thead>
        <tbody className="divide-y divide-line">
          {items.map((action) => {
            const statusKey = STATUS_KEY[action.status];

            return (
              <tr key={action.id} className="h-11">
                <td className="whitespace-nowrap px-4 py-2 font-mono text-[12px] tabular-nums">
                  {format.dateTime(action.started_at)}
                </td>
                <td className="px-3 py-2">
                  {t(MODE_KEY[action.mode])}
                </td>
                <td className="px-3 py-2 text-right tabular-nums">
                  {format.count(action.item_count)}
                </td>
                <td className="px-3 py-2 text-right font-mono text-[12px] tabular-nums">
                  {format.bytes(action.freed_bytes)}
                </td>
                <td className="px-3 py-2">
                  <Badge tone={STATUS_TONE[action.status] ?? "neutral"}>
                    {statusKey ? t(statusKey) : action.status}
                  </Badge>
                </td>
                <td className="px-4 py-2 text-right">
                  <Button
                    variant="outline"
                    disabled={!action.undoable}
                    title={
                      action.undoable
                        ? t("table.undoTooltip")
                        : t("table.undoUnavailable")
                    }
                    onClick={() => onUndo(action)}
                  >
                    {t("execute.undo")}
                  </Button>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
