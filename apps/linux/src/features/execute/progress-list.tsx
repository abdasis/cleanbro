import type { ReactElement } from "react";
import type { ItemProgress } from "../../hooks/use-execute";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { TranslationKey } from "../../lib/i18n/dictionary";
import type { CleanItemState, PlanItem } from "../../lib/types";

const STATE_KEY: Record<CleanItemState, TranslationKey> = {
  pending: "state.pending",
  running: "state.running",
  done: "state.done",
  failed: "state.failed",
  skipped: "state.skipped",
};

export type ProgressListProps = {
  items: PlanItem[];
  progress: Record<string, ItemProgress>;
};

export function ProgressList({
  items,
  progress,
}: ProgressListProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

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
                {format.truncateMiddle(item.path, 56)}
              </span>
              <span className="block text-[11px] text-ink-muted">
                {t(STATE_KEY[current])}
              </span>
            </span>
            <span className="shrink-0 font-mono text-[12px] text-ink-muted tabular-nums">
              {format.bytes(state?.freedBytes ?? item.bytes)}
            </span>
          </li>
        );
      })}
    </ul>
  );
}
