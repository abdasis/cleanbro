import type { ReactElement } from "react";
import { ChevronRight } from "lucide-react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { formatBytes, formatCount } from "../../lib/format";
import { RISK_LABEL, ruleMeta } from "../../lib/rules";
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
  if (categories.length === 0) {
    return (
      <div className="rounded-cb border border-line bg-surface px-6 py-8">
        <p className="font-semibold">
          {scanning ? "Sedang menghitung kategori..." : "Belum ada pemindaian"}
        </p>
        <p className="mt-1 text-ink-muted">
          {scanning
            ? "Hasil muncul begitu setiap kategori selesai."
            : "Jalankan scan cepat untuk melihat ruang yang bisa dibebaskan."}
        </p>
      </div>
    );
  }

  return (
    <ul className="divide-y divide-line overflow-hidden rounded-cb border border-line bg-surface">
      {categories.map((category) => {
        const meta = ruleMeta(category.rule_id);

        return (
          <li key={category.rule_id}>
            <button
              type="button"
              onClick={() => onSelect(category)}
              aria-label={`Tinjau ${meta.label}, ${formatBytes(category.bytes)}`}
              className="flex h-11 w-full items-center gap-3 px-4 text-left transition-colors duration-150 ease-cb hover:bg-surface-muted"
            >
              <span className="min-w-0 flex-1 truncate">{meta.label}</span>
              <span className="shrink-0 text-[12px] text-ink-muted tabular-nums">
                {formatCount(category.item_count)} item
              </span>
              <span className="w-24 shrink-0 text-right font-mono text-[13px] tabular-nums">
                {formatBytes(category.bytes)}
              </span>
              <Badge tone={RISK_TONE[meta.risk]}>
                {RISK_LABEL[meta.risk]}
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
