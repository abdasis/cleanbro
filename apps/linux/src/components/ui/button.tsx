import type {
  ComponentPropsWithoutRef,
  ReactElement,
  Ref,
} from "react";

type ButtonVariant = "primary" | "outline" | "ghost";

const VARIANTS: Record<ButtonVariant, string> = {
  primary: "border-ink bg-ink text-surface hover:opacity-90",
  outline: "border-line bg-surface text-ink hover:bg-surface-muted",
  ghost: "border-transparent bg-transparent text-ink hover:bg-surface-muted",
};

export type ButtonProps = ComponentPropsWithoutRef<"button"> & {
  variant?: ButtonVariant;
  /** React 19 meneruskan ref sebagai prop biasa. */
  ref?: Ref<HTMLButtonElement>;
};

/**
 * Kontrol 32px dengan area klik 40px (pseudo-element `after`).
 */
export function Button({
  variant = "outline",
  className = "",
  type = "button",
  ...props
}: ButtonProps): ReactElement {
  return (
    <button
      type={type}
      className={`relative inline-flex h-8 items-center justify-center gap-1.5 rounded-cb border px-3 text-[13px] font-medium transition duration-150 ease-cb after:absolute after:-inset-1 after:content-[''] active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50 ${VARIANTS[variant]} ${className}`}
      {...props}
    />
  );
}
