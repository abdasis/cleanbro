import type { ReactElement } from "react";

export type CheckboxProps = {
  checked: boolean;
  label: string;
  disabled?: boolean;
  onChange: (checked: boolean) => void;
};

/** Checkbox native dengan area klik 40px dan label aksesibel. */
export function Checkbox({
  checked,
  label,
  disabled = false,
  onChange,
}: CheckboxProps): ReactElement {
  return (
    <label className="relative inline-flex items-center justify-center before:absolute before:-inset-3 before:content-['']">
      <input
        type="checkbox"
        aria-label={label}
        checked={checked}
        disabled={disabled}
        onChange={(event) => onChange(event.target.checked)}
        className="h-4 w-4 cursor-pointer accent-ink disabled:cursor-not-allowed disabled:opacity-50"
      />
    </label>
  );
}
