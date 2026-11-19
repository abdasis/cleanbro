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

type View = "overview" | "history";

type NavItem = {
  id: View | "settings";
  label: string;
  icon: LucideIcon;
  enabled: boolean;
};

const NAV_ITEMS: NavItem[] = [
  { id: "overview", label: "Overview", icon: LayoutDashboard, enabled: true },
  { id: "history", label: "Riwayat", icon: History, enabled: true },
  { id: "settings", label: "Setelan", icon: Settings, enabled: false },
];

const ICON_PROPS = { size: 16, strokeWidth: 1.5 } as const;

const TAGLINE: Record<View, string> = {
  overview: "Pindai, tinjau, lalu bersihkan ruang disk.",
  history: "Catatan aksi pembersihan dan pemulihannya.",
};

export function AppLayout(): ReactElement {
  const [view, setView] = useState<View>("overview");

  return (
    <div className="flex h-dvh flex-col overflow-hidden border border-line bg-canvas text-ink">
      <Titlebar />

      <div className="flex min-h-0 flex-1">
        <aside className="flex w-56 shrink-0 flex-col border-r border-line bg-surface">
          <nav
            aria-label="Navigasi utama"
            className="flex flex-col gap-1 px-2 py-2"
          >
            {NAV_ITEMS.map((item) => {
              const Icon = item.icon;
              const active = item.id === view;

              return (
                <button
                  key={item.id}
                  type="button"
                  disabled={!item.enabled}
                  aria-current={active ? "page" : undefined}
                  title={item.enabled ? undefined : "Segera hadir"}
                  onClick={() => {
                    if (item.id === "overview" || item.id === "history") {
                      setView(item.id);
                    }
                  }}
                  className={`flex h-8 items-center gap-2 rounded-cb px-2 text-left transition-colors duration-150 ease-cb ${
                    active
                      ? "bg-surface-muted text-ink"
                      : item.enabled
                        ? "text-ink-muted hover:bg-surface-muted hover:text-ink"
                        : "text-ink-muted disabled:opacity-60"
                  }`}
                >
                  <Icon {...ICON_PROPS} aria-hidden="true" />
                  <span>{item.label}</span>
                  {item.enabled ? null : (
                    <span className="ml-auto text-[11px]">Segera</span>
                  )}
                </button>
              );
            })}
          </nav>
        </aside>

        <div className="flex min-w-0 flex-1 flex-col">
          <header className="flex h-14 shrink-0 items-center gap-4 border-b border-line bg-surface pl-6">
            {view === "overview" ? (
              <h1 className="text-balance text-2xl font-semibold tracking-[-0.02em]">
                Overview
              </h1>
            ) : (
              <nav aria-label="Breadcrumb">
                <ol className="flex items-center gap-1.5 text-lg font-semibold tracking-[-0.02em]">
                  <li>
                    <button
                      type="button"
                      onClick={() => setView("overview")}
                      className="rounded-cb text-ink-muted transition-colors duration-150 ease-cb hover:text-ink"
                    >
                      Overview
                    </button>
                  </li>
                  <li aria-hidden="true" className="text-ink-muted">
                    <ChevronRight size={16} strokeWidth={1.5} />
                  </li>
                  <li aria-current="page">Riwayat</li>
                </ol>
              </nav>
            )}
            <p className="truncate text-ink-muted">{TAGLINE[view]}</p>
          </header>

          <main className="flex-1 overflow-y-auto p-6">
            {view === "overview" ? (
              <OverviewPage onViewHistory={() => setView("history")} />
            ) : (
              <HistoryPage />
            )}
          </main>
        </div>
      </div>
    </div>
  );
}
