import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactElement,
} from "react";
import { useAppInfo } from "../../hooks/use-app-info";
import { useRules } from "../../hooks/use-rules";
import { useEnabledRules } from "../../lib/enabled-rules";
import { useI18n } from "../../lib/i18n/i18n-context";
import { AboutSection } from "./about-section";
import { CategorySection } from "./category-section";
import { DeleteModeSection } from "./delete-mode-section";
import { LanguageSection } from "./language-section";

const SAVED_MS = 2000;

export function SettingsPage(): ReactElement {
  const { t } = useI18n();
  const rules = useRules();
  const appInfo = useAppInfo();
  const { enabled, isEnabled, toggle, selectAll, clear } = useEnabledRules();
  const [saved, setSaved] = useState(false);
  const timerRef = useRef<number | null>(null);

  const notifySaved = useCallback(() => {
    setSaved(true);
    if (timerRef.current !== null) window.clearTimeout(timerRef.current);
    timerRef.current = window.setTimeout(() => {
      setSaved(false);
      timerRef.current = null;
    }, SAVED_MS);
  }, []);

  useEffect(
    () => () => {
      if (timerRef.current !== null) window.clearTimeout(timerRef.current);
    },
    [],
  );

  const allIds = useMemo(
    () => rules.data?.map((rule) => rule.id) ?? [],
    [rules.data],
  );

  const handleToggle = useCallback(
    (ruleId: string) => {
      toggle(ruleId, allIds);
      notifySaved();
    },
    [toggle, allIds, notifySaved],
  );

  const handleSelectAll = useCallback(() => {
    selectAll();
    notifySaved();
  }, [selectAll, notifySaved]);

  const handleClear = useCallback(() => {
    clear();
    notifySaved();
  }, [clear, notifySaved]);

  return (
    <div className="mx-auto flex w-full max-w-3xl flex-col gap-6">
      <div className="flex items-center justify-between gap-3">
        <p className="text-ink-muted">{t("settings.autoSaveHint")}</p>
        <p
          aria-live="polite"
          className="h-4 text-[12px] text-ink-muted"
        >
          {saved ? t("settings.saved") : ""}
        </p>
      </div>

      <LanguageSection onSaved={notifySaved} />

      <CategorySection
        rules={rules.data ?? []}
        isPending={rules.isPending}
        isError={rules.isError}
        error={rules.error}
        onRetry={() => void rules.refetch()}
        enabled={enabled}
        isEnabled={isEnabled}
        onToggle={handleToggle}
        onSelectAll={handleSelectAll}
        onClear={handleClear}
      />

      <DeleteModeSection />

      <AboutSection
        info={appInfo.data}
        isPending={appInfo.isPending}
        isError={appInfo.isError}
        error={appInfo.error}
        onRetry={() => void appInfo.refetch()}
      />
    </div>
  );
}
