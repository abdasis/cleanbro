import { useRef, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { Dialog } from "../../components/ui/dialog";
import { ErrorState } from "../../components/ui/error-state";
import { useDockerPrune } from "../../hooks/use-docker";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { DockerPruneTarget } from "../../lib/types";

const TARGETS: DockerPruneTarget[] = ["containers", "images_dangling"];

export type DockerPruneDialogProps = {
  open: boolean;
  onClose: () => void;
};

export function DockerPruneDialog({
  open,
  onClose,
}: DockerPruneDialogProps): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const prune = useDockerPrune();
  const cancelRef = useRef<HTMLButtonElement>(null);

  const running = prune.isPending;
  const done = prune.isSuccess;
  const actions = [
    t("docker.actionContainers"),
    t("docker.actionImages"),
  ];

  const close = () => {
    prune.reset();
    onClose();
  };

  const footer = done || prune.isError ? (
    <Button variant="outline" onClick={close}>
      {t("common.close")}
    </Button>
  ) : (
    <>
      <Button ref={cancelRef} variant="outline" onClick={close}>
        {t("common.cancel")}
      </Button>
      <Button
        variant="primary"
        disabled={running}
        onClick={() => prune.mutate(TARGETS)}
      >
        {running ? t("docker.cleaning") : t("docker.cleanAction")}
      </Button>
    </>
  );

  return (
    <Dialog
      open={open}
      title={t("docker.pruneTitle")}
      description={t("docker.pruneDesc")}
      size="sm"
      dismissible={!running}
      initialFocusRef={cancelRef}
      onClose={close}
      footer={footer}
    >
      {prune.isError ? (
        <ErrorState
          title={t("docker.pruneErrorTitle")}
          error={prune.error}
          onRetry={() => prune.mutate(TARGETS)}
        />
      ) : done && prune.data ? (
        <div>
          <p className="font-mono text-lg tabular-nums">
            {t("docker.freed", {
              size: format.bytes(prune.data.freed_bytes),
            })}
          </p>
          <p className="mt-1 text-ink-muted">
            {t("docker.deleted", {
              count: format.count(prune.data.deleted),
            })}
          </p>
        </div>
      ) : (
        <div className="flex flex-col gap-3">
          <ul className="flex flex-col gap-1.5">
            {actions.map((action) => (
              <li key={action} className="flex items-start gap-2">
                <span
                  aria-hidden="true"
                  className="mt-[7px] h-1 w-1 shrink-0 rounded-full bg-ink-muted"
                />
                {action}
              </li>
            ))}
          </ul>
          <p className="text-ink-muted">
            {running ? t("docker.pruneRunning") : t("docker.pruneHint")}
          </p>
        </div>
      )}
    </Dialog>
  );
}
