import type { ReactElement } from "react";
import { useI18n } from "../../lib/i18n/i18n-context";
import type { Language } from "../../lib/i18n/dictionary";
import { SettingsSection } from "./settings-section";

const OPTIONS: Array<{ value: Language; labelKey: "settings.languageId" | "settings.languageEn" }> = [
  { value: "id", labelKey: "settings.languageId" },
  { value: "en", labelKey: "settings.languageEn" },
];

export type LanguageSectionProps = {
  onSaved: () => void;
};

export function LanguageSection({
  onSaved,
}: LanguageSectionProps): ReactElement {
  const { t, language, setLanguage } = useI18n();

  return (
    <SettingsSection
      title={t("settings.language")}
      description={t("settings.languageDesc")}
    >
      <div
        role="radiogroup"
        aria-label={t("settings.language")}
        className="inline-flex rounded-cb border border-line p-0.5"
      >
        {OPTIONS.map((option) => {
          const active = language === option.value;

          return (
            <button
              key={option.value}
              type="button"
              role="radio"
              aria-checked={active}
              onClick={() => {
                if (active) return;
                setLanguage(option.value);
                onSaved();
              }}
              className={`h-8 rounded-[6px] px-3 text-[13px] transition-colors duration-150 ease-cb active:scale-[0.96] ${
                active
                  ? "bg-ink text-surface"
                  : "text-ink-muted hover:text-ink"
              }`}
            >
              {t(option.labelKey)}
            </button>
          );
        })}
      </div>
    </SettingsSection>
  );
}
