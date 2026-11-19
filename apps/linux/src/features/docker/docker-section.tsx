import { useState, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { useDockerStatus } from "../../hooks/use-docker";
import { toIpcError } from "../../lib/ipc";
import { formatBytes } from "../../lib/format";
import type { DockerDf } from "../../lib/types";
import { DockerContainerList, DockerImageList } from "./docker-lists";
import { DockerPruneDialog } from "./docker-prune-dialog";

function Unavailable({ reason }: { reason: string }): ReactElement {
  return (
    <div className="rounded-cb border border-line bg-surface px-6 py-5">
      <p className="font-semibold">Docker tidak tersedia</p>
      <p className="mt-1 text-ink-muted">{reason}</p>
    </div>
  );
}

function Stat({
  label,
  value,
}: {
  label: string;
  value: string;
}): ReactElement {
  return (
    <div>
      <dt className="text-[12px] text-ink-muted">{label}</dt>
      <dd className="font-mono text-[13px] tabular-nums">{value}</dd>
    </div>
  );
}

function Summary({ df }: { df: DockerDf }): ReactElement {
  return (
    <div className="flex flex-wrap items-end justify-between gap-4 border-b border-line px-4 py-4">
      <div>
        <p className="text-ink-muted">Ruang bisa dibebaskan</p>
        <p className="mt-1 font-mono text-2xl font-semibold tracking-[-0.02em] tabular-nums">
          {formatBytes(df.reclaimable_bytes)}
        </p>
      </div>
      <dl className="grid grid-cols-3 gap-4">
        <Stat
          label="Container aktif"
          value={`${df.containers_active} dari ${df.containers_total}`}
        />
        <Stat
          label="Image dipakai"
          value={`${df.images_active} dari ${df.images_total}`}
        />
        <Stat
          label="Volume lokal"
          value={`${df.local_volumes} dari ${df.volumes_total}`}
        />
      </dl>
    </div>
  );
}

export function DockerSection(): ReactElement {
  const status = useDockerStatus();
  const [dialogOpen, setDialogOpen] = useState(false);

  let content: ReactElement;

  if (status.isPending) {
    content = (
      <div className="rounded-cb border border-line bg-surface px-6 py-5 text-ink-muted">
        Memeriksa Docker...
      </div>
    );
  } else if (status.isError) {
    content = <Unavailable reason={toIpcError(status.error).message} />;
  } else if (!status.data.available) {
    content = (
      <Unavailable
        reason={
          status.data.reason ?? "Docker tidak terdeteksi di mesin ini."
        }
      />
    );
  } else {
    const { df, containers, images } = status.data;

    content = (
      <div className="overflow-hidden rounded-cb border border-line bg-surface">
        {df ? <Summary df={df} /> : null}
        <div className="divide-y divide-line">
          <DockerContainerList containers={containers} />
          <DockerImageList images={images} />
        </div>
        <div className="flex flex-wrap items-center justify-between gap-3 border-t border-line px-4 py-3">
          <p className="text-[12px] text-ink-muted">
            Hanya container berhenti dan image tanpa referensi.
          </p>
          <Button variant="outline" onClick={() => setDialogOpen(true)}>
            Bersihkan docker
          </Button>
        </div>
      </div>
    );
  }

  return (
    <section aria-labelledby="docker-section-heading">
      <div className="mb-2 flex items-center justify-between gap-3">
        <h2 id="docker-section-heading" className="font-semibold">
          Docker
        </h2>
        <p className="text-ink-muted">Adapter CLI, bukan filesystem</p>
      </div>
      {content}
      <DockerPruneDialog
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
      />
    </section>
  );
}
