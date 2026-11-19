import type { ReactElement } from "react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { Checkbox } from "../../components/ui/checkbox";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import { RISK_KEY } from "../../lib/rules";
import type { PlanItem, RiskLevel } from "../../lib/types";

const RISK_TONE: Record<RiskLevel, BadgeTone> = {
  safe: "neutral",
  review: "warn",
  risky: "danger",
};

export type ItemTableProps = {
  items: PlanItem[];
  disabled: boolean;
  onToggle: (item: PlanItem, selected: boolean) => void;
};

export function ItemTable({
  items,
  disabled,
  onToggle,
}: ItemTableProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

  return (
    <table className="w-full table-fixed">
      <caption className="sr-only">{t("itemTable.caption")}</caption>
      <thead>
        <tr className="border-b border-line text-left text-[11px] uppercase tracking-[0.06em] text-ink-muted">
          <th scope="col" className="w-12 py-2 pl-4 font-medium">
            {t("itemTable.select")}
          </th>
          <th scope="col" className="py-2 font-medium">
            {t("itemTable.pathReason")}
          </th>
          <th scope="col" className="w-24 py-2 pr-4 text-right font-medium">
            {t("itemTable.size")}
          </th>
        </tr>
      </thead>
      <tbody className="divide-y divide-line">
        {items.map((item) => {
          const selected = !item.excluded;

          return (
            <tr key={item.item_id} className="min-h-11 align-top">
              <td className="py-2 pl-4">
                <Checkbox
                  checked={selected}
                  label={t("itemTable.selectItem", { path: item.path })}
                  disabled={disabled}
                  onChange={(next) => onToggle(item, next)}
                />
              </td>
              <td className="min-w-0 py-2 pr-3">
                <p
                  title={item.path}
                  className="truncate font-mono text-[12px] leading-5"
                >
                  {format.truncateMiddle(item.path, 72)}
                </p>
                <p className="truncate leading-5 text-ink-muted">
                  {item.reason}
                </p>
              </td>
              <td className="py-2 pr-4 text-right">
                <span className="font-mono text-[12px] tabular-nums">
                  {format.bytes(item.bytes)}
                </span>
                <span className="mt-1 block">
                  <Badge tone={RISK_TONE[item.risk]}>
                    {t(RISK_KEY[item.risk])}
                  </Badge>
                </span>
              </td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
