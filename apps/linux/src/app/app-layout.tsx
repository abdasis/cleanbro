import { useState, type ReactElement } from "react";
import {
  ChevronRight,
  History,
  LayoutDashboard,
  Settings,
  type LucideIcon,
} from "lucide-react";
import { Titlebar } from "../components/titlebar";
import { HistoryPage } from "../features/history/history-page";
import { OverviewPage } from "../features/overview/overview-page";
import { SettingsPage } from "../features/settings/settings-page";
import { useI18n } from "../lib/i18n/i18n-context";
import type { TranslationKey } from "../lib/i18n/dictionary";

type View = "overview" | "history" | "settings";

type NavItem = {
  id: View;
  labelKey: TranslationKey;
  icon: LucideIcon;
};

const NAV_ITEMS: NavItem[] = [
  { id: "overview", labelKey: "nav.overview", icon: LayoutDashboard },
  { id: "history", labelKey: "nav.history", icon: History },
  { id: "settings", labelKey: "nav.settings", icon: Settings },
];

const ICON_PROPS = { size: 16, strokeWidth: 1.5 } as const;

export function AppLayout(): ReactElement {
  const { t } = useI18n();
  const [view, setView] = useState<View>("overview");

  let content: ReactElement;

  if (view === "overview") {
    content = <OverviewPage onViewHistory={() => setView("history")} />;
  } else if (view === "history") {
    content = <HistoryPage />;
  } else {
    content = <SettingsPage />;
  }

  return (
    <div className="flex h-dvh flex-col overflow-hidden border border-line bg-canvas text-ink">
      <Titlebar />

      <div className="flex min-h-0 flex-1">
        <aside className="flex w-56 shrink-0 flex-col border-r border-line bg-surface">
          <nav
            aria-label={t("nav.primary")}
            className="flex flex-col gap-1 px-2 py-2"
          >
            {NAV_ITEMS.map((item) => {
              const Icon = item.icon;
              const active = item.id === view;

              return (
                <button
                  key={item.id}
                  type="button"
                  aria-current={active ? "page" : undefined}
                  onClick={() => setView(item.id)}
                  className={`flex h-8 items-center gap-2 rounded-cb px-2 text-left transition-colors duration-150 ease-cb ${
                    active
                      ? "bg-surface-muted text-ink"
                      : "text-ink-muted hover:bg-surface-muted hover:text-ink"
                  }`}
                >
                  <Icon {...ICON_PROPS} aria-hidden="true" />
                  <span>{t(item.labelKey)}</span>
                </button>
              );
            })}
          </nav>
        </aside>

        <div className="flex min-w-0 flex-1 flex-col">
          <header className="flex h-14 shrink-0 items-center gap-4 border-b border-line bg-surface pl-6">
            {view === "overview" ? (
              <h1 className="text-balance text-2xl font-semibold tracking-[-0.02em]">
                {t("nav.overview")}
              </h1>
            ) : (
              <nav aria-label={t("nav.breadcrumb")}>
                <ol className="flex items-center gap-1.5 text-lg font-semibold tracking-[-0.02em]">
                  <li>
                    <button
                      type="button"
                      onClick={() => setView("overview")}
                      className="rounded-cb text-ink-muted transition-colors duration-150 ease-cb hover:text-ink"
                    >
                      {t("nav.overview")}
                    </button>
                  </li>
                  <li aria-hidden="true" className="text-ink-muted">
                    <ChevronRight size={16} strokeWidth={1.5} />
                  </li>
                  <li aria-current="page">{t(`nav.${view}`)}</li>
                </ol>
              </nav>
            )}
            <p className="truncate text-ink-muted">
              {t(`tagline.${view}`)}
            </p>
          </header>

          <main className="flex-1 overflow-y-auto p-6">{content}</main>
        </div>
      </div>
    </div>
  );
}
