import type { ReactElement } from "react";
import { useI18n } from "../../lib/i18n/i18n-context";
import { SettingsSection } from "./settings-section";

export function DeleteModeSection(): ReactElement {
  const { t } = useI18n();

  return (
    <SettingsSection
      title={t("settings.deleteMode")}
      description={t("settings.deleteModeDesc")}
    >
      <div
        role="radiogroup"
        aria-label={t("settings.deleteMode")}
        className="inline-flex rounded-cb border border-line p-0.5"
      >
        <button
          type="button"
          role="radio"
          aria-checked="true"
          className="h-8 rounded-[6px] bg-ink px-3 text-[13px] text-surface"
        >
          {t("deleteMode.trash")}
        </button>
        <button
          type="button"
          role="radio"
          aria-checked="false"
          disabled
          className="h-8 cursor-not-allowed rounded-[6px] px-3 text-[13px] text-ink-muted opacity-40"
        >
          {t("deleteMode.permanent")}
        </button>
      </div>
      <p className="mt-3 text-ink-muted">{t("settings.deleteModeNote")}</p>
    </SettingsSection>
  );
}
