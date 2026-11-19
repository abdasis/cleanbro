import {
  useEffect,
  useId,
  useRef,
  type ReactElement,
  type ReactNode,
  type RefObject,
} from "react";
import { X } from "lucide-react";

const FOCUSABLE =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

const WIDTH: Record<DialogSize, string> = {
  sm: "max-w-md",
  md: "max-w-lg",
  lg: "max-w-xl",
};

export type DialogSize = "sm" | "md" | "lg";

export type DialogProps = {
  open: boolean;
  title: string;
  description?: string;
  size?: DialogSize;
  dismissible?: boolean;
  /** Elemen fokus awal; default elemen fokusable pertama. */
  initialFocusRef?: RefObject<HTMLElement | null>;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
};

/**
 * Dialog tunggal: role dialog, aria-modal, Esc menutup, autofocus elemen
 * pertama, dan fokus berputar di dalam panel.
 */
export function Dialog({
  open,
  title,
  description,
  size = "md",
  dismissible = true,
  initialFocusRef,
  onClose,
  children,
  footer,
}: DialogProps): ReactElement | null {
  const titleId = useId();
  const descriptionId = useId();
  const panelRef = useRef<HTMLDivElement>(null);
  const onCloseRef = useRef(onClose);
  const dismissibleRef = useRef(dismissible);
  const restoreFocusRef = useRef<HTMLElement | null>(null);

  useEffect(() => {
    onCloseRef.current = onClose;
    dismissibleRef.current = dismissible;
  }, [onClose, dismissible]);

  useEffect(() => {
    if (!open) return;

    const panel = panelRef.current;
    restoreFocusRef.current =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;

    const initial =
      initialFocusRef?.current ??
      panel?.querySelector<HTMLElement>(FOCUSABLE);
    initial?.focus();

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" && dismissibleRef.current) {
        event.preventDefault();
        onCloseRef.current();
        return;
      }

      if (event.key !== "Tab" || !panel) return;

      const nodes = Array.from(
        panel.querySelectorAll<HTMLElement>(FOCUSABLE),
      ).filter((node) => node.offsetParent !== null);
      const first = nodes[0];
      const last = nodes[nodes.length - 1];
      if (!first || !last) return;

      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };

    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      restoreFocusRef.current?.focus();
    };
  }, [open, initialFocusRef]);

  if (!open) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-ink/25 p-4"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && dismissible) onClose();
      }}
    >
      <div
        ref={panelRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        aria-describedby={description ? descriptionId : undefined}
        className={`cb-dialog-enter flex max-h-[calc(100dvh-2rem)] w-full flex-col rounded-cb border border-line bg-surface ${WIDTH[size]}`}
      >
        <div className="flex items-start justify-between gap-3 border-b border-line px-5 py-4">
          <div className="min-w-0">
            <h2 id={titleId} className="text-[15px] font-semibold">
              {title}
            </h2>
            {description ? (
              <p id={descriptionId} className="mt-0.5 text-ink-muted">
                {description}
              </p>
            ) : null}
          </div>
          <button
            type="button"
            aria-label="Tutup dialog"
            disabled={!dismissible}
            onClick={onClose}
            className="relative -mr-1 inline-flex h-8 w-8 shrink-0 items-center justify-center rounded-cb text-ink-muted transition-colors duration-150 ease-cb before:absolute before:-inset-1 before:content-[''] hover:bg-surface-muted hover:text-ink active:scale-[0.96] disabled:pointer-events-none disabled:opacity-40"
          >
            <X size={16} strokeWidth={1.5} aria-hidden="true" />
          </button>
        </div>

        <div className="min-h-0 flex-1 overflow-y-auto px-5 py-4">
          {children}
        </div>

        {footer ? (
          <div className="flex items-center justify-end gap-2 border-t border-line px-5 py-3">
            {footer}
          </div>
        ) : null}
      </div>
    </div>
  );
}
