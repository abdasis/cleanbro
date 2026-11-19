import { useRef, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { Dialog } from "../../components/ui/dialog";
import { ErrorState } from "../../components/ui/error-state";
import { useDockerPrune } from "../../hooks/use-docker";
import { formatBytes, formatCount } from "../../lib/format";
import type { DockerPruneTarget } from "../../lib/types";

const TARGETS: DockerPruneTarget[] = ["containers", "images_dangling"];

const ACTIONS = [
  "Hentikan dan hapus container mati",
  "Hapus image tanpa referensi",
];

export type DockerPruneDialogProps = {
  open: boolean;
  onClose: () => void;
};

export function DockerPruneDialog({
  open,
  onClose,
}: DockerPruneDialogProps): ReactElement {
  const prune = useDockerPrune();
  const cancelRef = useRef<HTMLButtonElement>(null);

  const running = prune.isPending;
  const done = prune.isSuccess;

  const close = () => {
    prune.reset();
    onClose();
  };

  const footer = done || prune.isError ? (
    <Button variant="outline" onClick={close}>
      Tutup
    </Button>
  ) : (
    <>
      <Button ref={cancelRef} variant="outline" onClick={close}>
        Batal
      </Button>
      <Button
        variant="primary"
        disabled={running}
        onClick={() => prune.mutate(TARGETS)}
      >
        {running ? "Membersihkan..." : "Bersihkan"}
      </Button>
    </>
  );

  return (
    <Dialog
      open={open}
      title="Bersihkan Docker"
      description="Dua aksi aman berikut akan dijalankan."
      size="sm"
      dismissible={!running}
      initialFocusRef={cancelRef}
      onClose={close}
      footer={footer}
    >
      {prune.isError ? (
        <ErrorState
          title="Pembersihan Docker gagal"
          error={prune.error}
          onRetry={() => prune.mutate(TARGETS)}
        />
      ) : done && prune.data ? (
        <div>
          <p className="font-mono text-lg tabular-nums">
            {formatBytes(prune.data.freed_bytes)} dibebaskan
          </p>
          <p className="mt-1 text-ink-muted">
            {formatCount(prune.data.deleted)} objek dihapus.
          </p>
        </div>
      ) : (
        <div className="flex flex-col gap-3">
          <ul className="flex flex-col gap-1.5">
            {ACTIONS.map((action) => (
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
            {running
              ? "Sedang menjalankan pembersihan. Tunggu sampai selesai."
              : "Container yang sudah berhenti akan hilang permanen. Volume tidak disentuh."}
          </p>
        </div>
      )}
    </Dialog>
  );
}
