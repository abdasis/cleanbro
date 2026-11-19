import type { ReactElement } from "react";
import type { ItemProgress } from "../../hooks/use-execute";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { CleanFinished, PlanItem } from "../../lib/types";

export type ResultSummaryProps = {
  finished: CleanFinished;
  durationMs: number | null;
  items: PlanItem[];
  progress: Record<string, ItemProgress>;
};

export function ResultSummary({
  finished,
  durationMs,
  items,
  progress,
}: ResultSummaryProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

  const problems = items.filter((item) => {
    const state = progress[item.item_id]?.state;
    return state === "failed" || state === "skipped";
  });

  const stats: Array<{ label: string; value: string }> = [
    { label: t("result.freed"), value: format.bytes(finished.freed_bytes) },
    { label: t("result.deleted"), value: format.count(finished.deleted) },
    { label: t("result.failed"), value: format.count(finished.failed) },
    { label: t("result.skipped"), value: format.count(finished.skipped) },
    {
      label: t("result.duration"),
      value: durationMs === null ? "-" : format.duration(durationMs),
    },
  ];

  return (
    <div className="flex flex-col gap-4">
      <dl className="grid grid-cols-2 gap-3 sm:grid-cols-5">
        {stats.map((stat) => (
          <div
            key={stat.label}
            className="rounded-cb border border-line px-3 py-2"
          >
            <dt className="text-[11px] text-ink-muted">{stat.label}</dt>
            <dd className="mt-0.5 font-mono text-[13px] tabular-nums">
              {stat.value}
            </dd>
          </div>
        ))}
      </dl>

      {problems.length > 0 ? (
        <div>
          <h3 className="mb-2 font-semibold">{t("result.problems")}</h3>
          <table className="w-full table-fixed">
            <thead>
              <tr className="border-b border-line text-left text-[11px] uppercase tracking-[0.06em] text-ink-muted">
                <th scope="col" className="py-2 font-medium">
                  {t("result.path")}
                </th>
                <th scope="col" className="w-40 py-2 font-medium">
                  {t("result.reason")}
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-line">
              {problems.map((item) => {
                const state = progress[item.item_id]?.state;
                return (
                  <tr key={item.item_id}>
                    <td className="min-w-0 py-2 pr-3">
                      <span
                        title={item.path}
                        className="block truncate font-mono text-[12px]"
                      >
                        {format.truncateMiddle(item.path, 56)}
                      </span>
                    </td>
                    <td className="py-2 text-ink-muted">
                      {state === "skipped"
                        ? t("result.skippedReason")
                        : t("result.failedReason")}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      ) : null}
    </div>
  );
}
