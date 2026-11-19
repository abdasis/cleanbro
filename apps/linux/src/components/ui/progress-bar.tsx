import type { ReactElement } from "react";

export type ProgressBarProps = {
  /** `undefined` = indeterminate (total belum diketahui). */
  value?: number;
  label: string;
  className?: string;
};

export function ProgressBar({
  value,
  label,
  className = "",
}: ProgressBarProps): ReactElement {
  const clamped =
    value === undefined ? undefined : Math.min(100, Math.max(0, value));

  return (
    <div
      role="progressbar"
      aria-label={label}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={clamped}
      aria-valuetext={clamped === undefined ? "Sedang menghitung" : undefined}
      className={`h-1.5 w-full overflow-hidden rounded-full bg-surface-muted ${className}`}
    >
      {clamped === undefined ? (
        <div className="h-full w-1/3 rounded-full bg-ink-muted/40" />
      ) : (
        <div
          className="h-full rounded-full bg-ink"
          style={{ width: `${clamped}%` }}
        />
      )}
    </div>
  );
}
