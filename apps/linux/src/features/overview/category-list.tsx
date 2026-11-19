import type { ReactElement } from "react";
import { ChevronRight } from "lucide-react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import { RISK_KEY, ruleMeta } from "../../lib/rules";
import type { CategoryResult, RiskLevel } from "../../lib/types";

const RISK_TONE: Record<RiskLevel, BadgeTone> = {
  safe: "neutral",
  review: "warn",
  risky: "danger",
};

export type CategoryListProps = {
  categories: CategoryResult[];
  scanning: boolean;
  onSelect: (category: CategoryResult) => void;
};

export function CategoryList({
  categories,
  scanning,
  onSelect,
}: CategoryListProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();

  if (categories.length === 0) {
    return (
      <div className="rounded-cb border border-line bg-surface px-6 py-8">
        <p className="font-semibold">
          {scanning
            ? t("categoryList.scanning")
            : t("categoryList.empty")}
        </p>
        <p className="mt-1 text-ink-muted">
          {scanning
            ? t("categoryList.scanningHint")
            : t("categoryList.emptyHint")}
        </p>
      </div>
    );
  }

  return (
    <ul className="divide-y divide-line overflow-hidden rounded-cb border border-line bg-surface">
      {categories.map((category) => {
        const meta = ruleMeta(category.rule_id);
        const label = t(meta.labelKey);
        const size = format.bytes(category.bytes);

        return (
          <li key={category.rule_id}>
            <button
              type="button"
              onClick={() => onSelect(category)}
              aria-label={t("categoryList.review", { label, size })}
              className="flex h-11 w-full items-center gap-3 px-4 text-left transition-colors duration-150 ease-cb hover:bg-surface-muted"
            >
              <span className="min-w-0 flex-1 truncate">{label}</span>
              <span className="shrink-0 text-[12px] text-ink-muted tabular-nums">
                {t("common.itemCount", {
                  count: format.count(category.item_count),
                })}
              </span>
              <span className="w-24 shrink-0 text-right font-mono text-[13px] tabular-nums">
                {size}
              </span>
              <Badge tone={RISK_TONE[meta.risk]}>
                {t(RISK_KEY[meta.risk])}
              </Badge>
              <ChevronRight
                size={16}
                strokeWidth={1.5}
                aria-hidden="true"
                className="text-ink-muted"
              />
            </button>
          </li>
        );
      })}
    </ul>
  );
}
