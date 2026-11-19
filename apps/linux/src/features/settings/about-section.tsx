import type { ReactElement } from "react";
import { ErrorState } from "../../components/ui/error-state";
import { useI18n } from "../../lib/i18n/i18n-context";
import type { AppInfo } from "../../lib/types";
import { SettingsSection } from "./settings-section";

export type AboutSectionProps = {
  info: AppInfo | undefined;
  isPending: boolean;
  isError: boolean;
  error: unknown;
  onRetry: () => void;
};

export function AboutSection({
  info,
  isPending,
  isError,
  error,
  onRetry,
}: AboutSectionProps): ReactElement {
  const { t } = useI18n();

  let body: ReactElement;

  if (isPending) {
    body = <p className="text-ink-muted">{t("settings.aboutLoading")}</p>;
  } else if (isError) {
    body = (
      <ErrorState
        title={t("settings.aboutError")}
        error={error}
        onRetry={onRetry}
      />
    );
  } else if (info) {
    const rows: Array<{ label: string; value: string }> = [
      { label: t("settings.aboutName"), value: info.name },
      { label: t("settings.aboutVersion"), value: info.version },
      { label: t("settings.aboutPlatform"), value: info.platform },
      { label: t("settings.aboutRulesVersion"), value: info.rules_version },
    ];

    body = (
      <dl className="grid gap-x-6 gap-y-2 sm:grid-cols-2">
        {rows.map((row) => (
          <div key={row.label} className="flex items-baseline justify-between gap-4">
            <dt className="text-ink-muted">{row.label}</dt>
            <dd className="font-mono text-[13px] tabular-nums">
              {row.value}
            </dd>
          </div>
        ))}
      </dl>
    );
  } else {
    body = <p className="text-ink-muted">{t("settings.aboutError")}</p>;
  }

  return (
    <SettingsSection title={t("settings.about")}>{body}</SettingsSection>
  );
}
