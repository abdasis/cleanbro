import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";
import type { ReactElement } from "react";
import { useI18n } from "../lib/i18n/i18n-context";
import { isTauri } from "../lib/ipc";

const ICON_SIZE = 14;

/**
 * Kontrol jendela untuk titlebar frameless. Di browser biasa (tanpa Tauri)
 * komponen ini tidak dirender.
 */
export function WindowControls(): ReactElement | null {
  const { t } = useI18n();

  if (!isTauri()) return null;

  const appWindow = getCurrentWindow();
  const run = (action: () => Promise<void>) => () => {
    action().catch(() => undefined);
  };

  const buttonClass =
    "flex h-10 w-11 items-center justify-center text-ink-muted transition-colors duration-150 ease-cb hover:bg-surface-muted hover:text-ink active:scale-[0.96]";

  return (
    <div className="ml-auto flex h-full items-center">
      <button
        type="button"
        aria-label={t("titlebar.minimize")}
        title={t("titlebar.minimize")}
        className={buttonClass}
        onClick={run(() => appWindow.minimize())}
      >
        <Minus size={ICON_SIZE} strokeWidth={1.5} aria-hidden="true" />
      </button>
      <button
        type="button"
        aria-label={t("titlebar.maximize")}
        title={t("titlebar.maximize")}
        className={buttonClass}
        onClick={run(() => appWindow.toggleMaximize())}
      >
        <Square size={12} strokeWidth={1.5} aria-hidden="true" />
      </button>
      <button
        type="button"
        aria-label={t("titlebar.close")}
        title={t("titlebar.close")}
        className={buttonClass}
        onClick={run(() => appWindow.close())}
      >
        <X size={ICON_SIZE} strokeWidth={1.5} aria-hidden="true" />
      </button>
    </div>
  );
}
