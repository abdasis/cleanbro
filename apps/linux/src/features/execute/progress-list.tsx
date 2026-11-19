import type { ReactElement } from "react";
import type { ItemProgress } from "../../hooks/use-execute";
import { formatBytes, truncateMiddle } from "../../lib/format";
import type { CleanItemState, PlanItem } from "../../lib/types";

const STATE_LABEL: Record<CleanItemState, string> = {
  pending: "Menunggu",
  running: "Berjalan",
  done: "Selesai",
  failed: "Gagal",
  skipped: "Dilewati",
};

export type ProgressListProps = {
  items: PlanItem[];
  progress: Record<string, ItemProgress>;
};

export function ProgressList({
  items,
  progress,
}: ProgressListProps): ReactElement {
  return (
    <ul className="divide-y divide-line rounded-cb border border-line">
      {items.map((item) => {
        const state = progress[item.item_id];
        const current = state?.state ?? "pending";

        return (
          <li
            key={item.item_id}
            className="flex min-h-11 items-center gap-3 px-3 py-2"
          >
            <span className="min-w-0 flex-1">
              <span
                title={item.path}
                className="block truncate font-mono text-[12px]"
              >
                {truncateMiddle(item.path, 56)}
              </span>
              <span className="block text-[11px] text-ink-muted">
                {STATE_LABEL[current]}
              </span>
            </span>
            <span className="shrink-0 font-mono text-[12px] text-ink-muted tabular-nums">
              {formatBytes(state?.freedBytes ?? item.bytes)}
            </span>
          </li>
        );
      })}
    </ul>
  );
}
