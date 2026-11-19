import type { ReactElement } from "react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { Button } from "../../components/ui/button";
import { Checkbox } from "../../components/ui/checkbox";
import { ErrorState } from "../../components/ui/error-state";
import { useI18n } from "../../lib/i18n/i18n-context";
import { categoryKey, ruleKey } from "../../lib/i18n/dictionary";
import { RISK_KEY } from "../../lib/rules";
import type { RiskLevel, RuleCategory, RuleInfo } from "../../lib/types";
import { SettingsSection } from "./settings-section";

const RISK_TONE: Record<RiskLevel, BadgeTone> = {
  safe: "neutral",
  review: "warn",
  risky: "danger",
};

const CATEGORY_ORDER: RuleCategory[] = [
  "developer-cache",
  "build",
  "media",
  "docker",
  "system",
];

function categoryRank(category: string): number {
  const index = CATEGORY_ORDER.indexOf(category as RuleCategory);
  return index === -1 ? CATEGORY_ORDER.length : index;
}

function groupRules(rules: RuleInfo[]): Array<[string, RuleInfo[]]> {
  const groups = new Map<string, RuleInfo[]>();

  for (const rule of rules) {
    const list = groups.get(rule.category);
    if (list) list.push(rule);
    else groups.set(rule.category, [rule]);
  }

  return Array.from(groups.entries()).sort(
    ([left], [right]) => categoryRank(left) - categoryRank(right),
  );
}

export type CategorySectionProps = {
  rules: RuleInfo[];
  isPending: boolean;
  isError: boolean;
  error: unknown;
  onRetry: () => void;
  enabled: string[] | null;
  isEnabled: (ruleId: string) => boolean;
  onToggle: (ruleId: string) => void;
  onSelectAll: () => void;
  onClear: () => void;
};

export function CategorySection({
  rules,
  isPending,
  isError,
  error,
  onRetry,
  enabled,
  isEnabled,
  onToggle,
  onSelectAll,
  onClear,
}: CategorySectionProps): ReactElement {
  const { t } = useI18n();
  const groups = groupRules(rules);
  const allActive =
    enabled === null ||
    (rules.length > 0 && rules.every((rule) => enabled.includes(rule.id)));
  const noneActive = enabled !== null && enabled.length === 0;

  let body: ReactElement;

  if (isPending) {
    body = <p className="text-ink-muted">{t("settings.categoriesLoading")}</p>;
  } else if (isError) {
    body = (
      <ErrorState
        title={t("settings.categoriesError")}
        error={error}
        onRetry={onRetry}
      />
    );
  } else if (groups.length === 0) {
    body = <p className="text-ink-muted">{t("settings.categoriesEmpty")}</p>;
  } else {
    body = (
      <div className="flex flex-col gap-5">
        {groups.map(([category, categoryRules]) => (
          <div key={category}>
            <p className="mb-1 text-[11px] font-medium uppercase tracking-[0.06em] text-ink-muted">
              {t(categoryKey(category))}
            </p>
            <ul className="divide-y divide-line">
              {categoryRules.map((rule) => {
                const label = t(ruleKey(rule.id));

                return (
                  <li
                    key={rule.id}
                    className="flex min-h-10 items-center gap-3 py-1"
                  >
                    <Checkbox
                      checked={isEnabled(rule.id)}
                      label={t("settings.toggleRule", { rule: label })}
                      onChange={() => onToggle(rule.id)}
                    />
                    <span className="min-w-0 flex-1 truncate">{label}</span>
                    <code className="shrink-0 font-mono text-[12px] text-ink-muted">
                      {rule.id}
                    </code>
                    <Badge tone={RISK_TONE[rule.risk]}>
                      {t(RISK_KEY[rule.risk])}
                    </Badge>
                  </li>
                );
              })}
            </ul>
          </div>
        ))}
      </div>
    );
  }

  return (
    <SettingsSection
      title={t("settings.categories")}
      description={t("settings.categoriesDesc")}
      action={
        <div className="flex shrink-0 items-center gap-1">
          <Button
            variant="ghost"
            disabled={allActive || isPending || isError}
            onClick={onSelectAll}
          >
            {t("common.selectAll")}
          </Button>
          <Button
            variant="ghost"
            disabled={noneActive || isPending || isError}
            onClick={onClear}
          >
            {t("common.clear")}
          </Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {noneActive ? (
          <p className="text-ink-muted">{t("settings.categoriesNone")}</p>
        ) : null}
        {body}
      </div>
    </SettingsSection>
  );
}
