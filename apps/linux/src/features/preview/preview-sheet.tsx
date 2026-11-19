import { useEffect, useRef, useState, type ReactElement } from "react";
import { X } from "lucide-react";
import { Badge, type BadgeTone } from "../../components/ui/badge";
import { Button } from "../../components/ui/button";
import { ErrorState } from "../../components/ui/error-state";
import { useCleanPlan } from "../../hooks/use-clean-plan";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import { RISK_KEY, ruleMeta } from "../../lib/rules";
import type { PlanItem, RiskLevel } from "../../lib/types";
import { ExecuteDialog } from "../execute/execute-dialog";
import { ItemTable } from "./item-table";

const RISK_TONE: Record<RiskLevel, BadgeTone> = {
  safe: "neutral",
  review: "warn",
  risky: "danger",
};

export type PreviewSheetProps = {
  scanId: string;
  ruleId: string;
  onClose: () => void;
  onViewHistory: () => void;
};

export function PreviewSheet({
  scanId,
  ruleId,
  onClose,
  onViewHistory,
}: PreviewSheetProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const closeRef = useRef<HTMLButtonElement>(null);
  const [executeOpen, setExecuteOpen] = useState(false);

  const ruleIds = [ruleId];
  const { query, setExcluded } = useCleanPlan(scanId, ruleIds);
  const plan = query.data ?? null;
  const meta = ruleMeta(ruleId);
  const label = t(meta.labelKey);

  useEffect(() => {
    closeRef.current?.focus();
  }, []);

  useEffect(() => {
    if (executeOpen) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onClose();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [executeOpen, onClose]);

  const items = plan?.items ?? [];
  const selectedItems = items.filter((item) => !item.excluded);
  const safeExcluded = items.filter(
    (item) => item.risk === "safe" && item.excluded,
  );
  const canSelectSafe = safeExcluded.length > 0 && !setExcluded.isPending;
  const canExecute =
    selectedItems.length > 0 && plan !== null && !setExcluded.isPending;

  const toggleItem = (item: PlanItem, selected: boolean) => {
    setExcluded.mutate({ itemId: item.item_id, excluded: !selected });
  };

  const selectAllSafe = async () => {
    for (const item of safeExcluded) {
      await setExcluded.mutateAsync({
        itemId: item.item_id,
        excluded: false,
      });
    }
  };

  return (
    <div
      className="fixed inset-0 z-40 flex justify-end bg-ink/20"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !executeOpen) onClose();
      }}
    >
      <section
        role="region"
        aria-label={t("preview.regionAria", { label })}
        className="cb-sheet-enter flex h-full w-full flex-col border-l border-line bg-surface sm:w-[560px]"
      >
        <header className="shrink-0 border-b border-line px-6 py-4">
          <div className="flex items-start justify-between gap-3">
            <div className="min-w-0">
              <p className="text-ink-muted">{t("preview.label")}</p>
              <h2 className="text-lg font-semibold tracking-[-0.01em]">
                {label}
              </h2>
              <div className="mt-2 flex flex-wrap items-center gap-2">
                <code className="font-mono text-[12px] text-ink-muted">
                  {ruleId}
                </code>
                <Badge tone={RISK_TONE[meta.risk]}>
                  {t(RISK_KEY[meta.risk])}
                </Badge>
              </div>
            </div>
            <button
              ref={closeRef}
              type="button"
              aria-label={t("preview.close")}
              onClick={onClose}
              className="relative -mr-1 inline-flex h-8 w-8 shrink-0 items-center justify-center rounded-cb text-ink-muted transition-colors duration-150 ease-cb before:absolute before:-inset-1 before:content-[''] hover:bg-surface-muted hover:text-ink active:scale-[0.96]"
            >
              <X size={16} strokeWidth={1.5} aria-hidden="true" />
            </button>
          </div>
        </header>

        <div className="min-h-0 flex-1 overflow-y-auto">
          {query.isPending ? (
            <p className="px-6 py-8 text-ink-muted">{t("preview.loading")}</p>
          ) : query.isError ? (
            <div className="p-4">
              <ErrorState
                title={t("preview.errorTitle")}
                error={query.error}
                onRetry={() => void query.refetch()}
              />
            </div>
          ) : plan ? (
            <>
              <div className="flex items-center justify-between gap-4 px-6 py-4">
                <p className="text-ink-muted tabular-nums">
                  {t("preview.summary", {
                    count: format.count(plan.total_items),
                    size: format.bytes(plan.total_bytes),
                  })}
                </p>
                <span className="font-mono text-lg tabular-nums">
                  {format.bytes(plan.total_bytes)}
                </span>
              </div>

              {setExcluded.isError ? (
                <p className="px-6 pb-3 text-danger">
                  {t("preview.saveError")}
                </p>
              ) : null}

              <ItemTable
                items={items}
                disabled={setExcluded.isPending}
                onToggle={toggleItem}
              />
            </>
          ) : null}
        </div>

        <footer className="flex shrink-0 items-center gap-2 border-t border-line px-6 py-3">
          <Button
            variant="ghost"
            disabled={!canSelectSafe}
            onClick={() => void selectAllSafe()}
          >
            {t("preview.selectAllSafe")}
          </Button>
          <div className="ml-auto flex items-center gap-2">
            <Button variant="outline" onClick={onClose}>
              {t("common.cancel")}
            </Button>
            <Button
              variant="primary"
              disabled={!canExecute}
              onClick={() => setExecuteOpen(true)}
            >
              {t("preview.clean", {
                size: format.bytes(plan?.total_bytes ?? 0),
              })}
            </Button>
          </div>
        </footer>
      </section>

      {plan && executeOpen ? (
        <ExecuteDialog
          scanId={scanId}
          items={selectedItems}
          requiresConfirmation={plan.requires_confirmation}
          onClose={() => setExecuteOpen(false)}
          onFinish={() => {
            setExecuteOpen(false);
            onClose();
          }}
          onViewHistory={() => {
            setExecuteOpen(false);
            onViewHistory();
          }}
        />
      ) : null}
    </div>
  );
}
