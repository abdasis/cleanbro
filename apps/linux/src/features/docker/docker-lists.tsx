import type { ReactElement } from "react";
import { Badge } from "../../components/ui/badge";
import { useI18n } from "../../lib/i18n/i18n-context";
import { useFormat } from "../../lib/i18n/use-format";
import type { DockerContainer, DockerImage } from "../../lib/types";

const LIST_LIMIT = 5;

function ListHeader({
  title,
  count,
}: {
  title: string;
  count: number;
}): ReactElement {
  return (
    <div className="flex items-center justify-between gap-3 border-b border-line px-4 py-2">
      <h3 className="font-semibold">{title}</h3>
      <span className="text-[12px] text-ink-muted tabular-nums">{count}</span>
    </div>
  );
}

function Remaining({ count }: { count: number }): ReactElement | null {
  const { t } = useI18n();
  if (count <= 0) return null;
  return (
    <p className="border-t border-line px-4 py-2 text-[12px] text-ink-muted">
      {t("docker.remaining", { count })}
    </p>
  );
}

function Empty({ label }: { label: string }): ReactElement {
  return <p className="px-4 py-3 text-ink-muted">{label}</p>;
}

export function DockerContainerList({
  containers,
}: {
  containers: DockerContainer[];
}): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const visible = containers.slice(0, LIST_LIMIT);

  return (
    <div>
      <ListHeader title={t("docker.containersTitle")} count={containers.length} />
      {visible.length === 0 ? (
        <Empty label={t("docker.noContainers")} />
      ) : (
        <ul className="divide-y divide-line">
          {visible.map((container) => (
            <li
              key={container.id}
              className="flex h-11 items-center gap-2 px-4"
            >
              <span className="min-w-0 flex-1 truncate" title={container.name}>
                {container.name}
              </span>
              <span
                className="hidden max-w-[38%] truncate font-mono text-[12px] text-ink-muted sm:block"
                title={container.image}
              >
                {container.image}
              </span>
              <span className="w-16 shrink-0 text-right text-[12px] text-ink-muted">
                {container.state}
              </span>
              <span className="w-20 shrink-0 text-right font-mono text-[12px] tabular-nums">
                {format.bytes(container.size_bytes)}
              </span>
            </li>
          ))}
        </ul>
      )}
      <Remaining count={containers.length - visible.length} />
    </div>
  );
}

export function DockerImageList({
  images,
}: {
  images: DockerImage[];
}): ReactElement {
  const { t } = useI18n();
  const format = useFormat();
  const visible = images.slice(0, LIST_LIMIT);

  return (
    <div>
      <ListHeader title={t("docker.imagesTitle")} count={images.length} />
      {visible.length === 0 ? (
        <Empty label={t("docker.noImages")} />
      ) : (
        <ul className="divide-y divide-line">
          {visible.map((image) => {
            const reference = `${image.repository}:${image.tag}`;

            return (
              <li key={image.id} className="flex h-11 items-center gap-2 px-4">
                <span
                  className="min-w-0 flex-1 truncate font-mono text-[12px]"
                  title={reference}
                >
                  {reference}
                </span>
                {image.in_use ? <Badge>{t("docker.inUse")}</Badge> : null}
                <span className="w-20 shrink-0 text-right font-mono text-[12px] tabular-nums">
                  {format.bytes(image.size_bytes)}
                </span>
              </li>
            );
          })}
        </ul>
      )}
      <Remaining count={images.length - visible.length} />
    </div>
  );
}
