import { useState, type ReactElement } from "react";
import { Button } from "../../components/ui/button";
import { useDockerStatus } from "../../hooks/use-docker";
import { useErrorMessage, useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { DockerDf } from "../../lib/types";
import { DockerContainerList, DockerImageList } from "./docker-lists";
import { DockerPruneDialog } from "./docker-prune-dialog";

function Unavailable({ reason }: { reason: string }): ReactElement {
  const { t } = useI18n();

  return (
    <div className="rounded-cb border border-line bg-surface px-6 py-5">
      <p className="font-semibold">{t("docker.unavailable")}</p>
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
  const { t } = useI18n();
  const format = useFormat();

  return (
    <div className="flex flex-wrap items-end justify-between gap-4 border-b border-line px-4 py-4">
      <div>
        <p className="text-ink-muted">{t("docker.reclaimable")}</p>
        <p className="mt-1 font-mono text-2xl font-semibold tracking-[-0.02em] tabular-nums">
          {format.bytes(df.reclaimable_bytes)}
        </p>
      </div>
      <dl className="grid grid-cols-3 gap-4">
        <Stat
          label={t("docker.containersActive")}
          value={t("docker.of", {
            active: format.count(df.containers_active),
            total: format.count(df.containers_total),
          })}
        />
        <Stat
          label={t("docker.imagesUsed")}
          value={t("docker.of", {
            active: format.count(df.images_active),
            total: format.count(df.images_total),
          })}
        />
        <Stat
          label={t("docker.localVolumes")}
          value={t("docker.of", {
            active: format.count(df.local_volumes),
            total: format.count(df.volumes_total),
          })}
        />
      </dl>
    </div>
  );
}

export function DockerSection(): ReactElement {
  const { t } = useI18n();
  const errorMessage = useErrorMessage();
  const status = useDockerStatus();
  const [dialogOpen, setDialogOpen] = useState(false);

  let content: ReactElement;

  if (status.isPending) {
    content = (
      <div className="rounded-cb border border-line bg-surface px-6 py-5 text-ink-muted">
        {t("docker.checking")}
      </div>
    );
  } else if (status.isError) {
    content = <Unavailable reason={errorMessage(status.error)} />;
  } else if (!status.data.available) {
    content = (
      <Unavailable reason={status.data.reason ?? t("docker.notDetected")} />
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
          <p className="text-[12px] text-ink-muted">{t("docker.hint")}</p>
          <Button variant="outline" onClick={() => setDialogOpen(true)}>
            {t("docker.clean")}
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
        <p className="text-ink-muted">{t("docker.sectionHint")}</p>
      </div>
      {content}
      <DockerPruneDialog
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
      />
    </section>
  );
}
