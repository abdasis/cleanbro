import type { ReactElement, ReactNode } from "react";

export type SettingsSectionProps = {
  title: string;
  description?: string;
  action?: ReactNode;
  children: ReactNode;
};

export function SettingsSection({
  title,
  description,
  action,
  children,
}: SettingsSectionProps): ReactElement {
  return (
    <section className="rounded-cb border border-line bg-surface">
      <header className="flex items-start justify-between gap-3 border-b border-line px-6 py-4">
        <div className="min-w-0">
          <h2 className="font-semibold">{title}</h2>
          {description ? (
            <p className="mt-0.5 text-ink-muted">{description}</p>
          ) : null}
        </div>
        {action}
      </header>
      <div className="px-6 py-4">{children}</div>
    </section>
  );
}
