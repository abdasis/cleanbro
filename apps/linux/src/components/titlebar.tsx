import { HardDrive } from "lucide-react";
import type { ReactElement } from "react";
import { useI18n } from "../lib/i18n/i18n-context";
import { Badge } from "./ui/badge";
import { WindowControls } from "./window-controls";

/**
 * Frame jendela kustom: satu baris penuh di atas konten. Seluruh area kecuali
 * tombol adalah drag region, jadi jendela bisa digeser dari mana saja di sini.
 */
export function Titlebar(): ReactElement {
  const { t } = useI18n();

  return (
    <div
      data-tauri-drag-region
      className="flex h-10 shrink-0 select-none items-center gap-2 border-b border-line bg-surface pl-3"
    >
      <HardDrive
        size={15}
        strokeWidth={1.5}
        aria-hidden="true"
        className="text-ink-muted"
      />
      <span data-tauri-drag-region className="text-[13px] font-medium">
        {t("app.name")}
      </span>
      <Badge className="ml-1">Linux</Badge>
      <WindowControls />
    </div>
  );
}
