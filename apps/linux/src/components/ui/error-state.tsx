import type { ReactElement } from "react";
import { useErrorMessage, useI18n } from "../../lib/i18n/i18n-context";
import { toIpcError } from "../../lib/ipc";
import { Button } from "./button";

export type ErrorStateProps = {
  title: string;
  error: unknown;
  onRetry: () => void;
};

export function ErrorState({
  title,
  error,
  onRetry,
}: ErrorStateProps): ReactElement {
  const { t } = useI18n();
  const errorMessage = useErrorMessage();
  const ipcError = toIpcError(error);

  return (
    <div className="rounded-cb border border-line bg-surface px-6 py-5">
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0">
          <p className="font-semibold">{title}</p>
          <p className="mt-1 text-ink-muted">{errorMessage(error)}</p>
          <code className="mt-2 inline-block font-mono text-[11px] text-ink-muted">
            {ipcError.code}
          </code>
        </div>
        <Button variant="outline" onClick={onRetry}>
          {t("common.retry")}
        </Button>
      </div>
    </div>
  );
}
