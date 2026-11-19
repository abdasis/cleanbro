import type { ReactElement, ReactNode } from "react";

export type BadgeTone = "neutral" | "warn" | "danger";

const TONES: Record<BadgeTone, string> = {
  neutral: "border-line text-ink-muted",
  warn: "border-warn/50 text-warn",
  danger: "border-danger/50 text-danger",
};

export type BadgeProps = {
  tone?: BadgeTone;
  children: ReactNode;
  className?: string;
};

export function Badge({
  tone = "neutral",
  children,
  className = "",
}: BadgeProps): ReactElement {
  return (
    <span
      className={`inline-flex h-5 shrink-0 items-center rounded-full border px-2 text-[11px] font-medium leading-none ${TONES[tone]} ${className}`}
    >
      {children}
    </span>
  );
}
