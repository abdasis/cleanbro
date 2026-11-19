import type { ReactElement } from "react";

export type AppShellProps = {
  product: string;
  platform: "linux" | "macos";
  tagline: string;
  version?: string;
};

const PLATFORM_LABEL: Record<AppShellProps["platform"], string> = {
  linux: "Linux",
  macos: "macOS",
};

export function AppShell({
  product,
  platform,
  tagline,
  version,
}: AppShellProps): ReactElement {
  return (
    <div className="cb-shell">
      <header className="cb-header">
        <div className="cb-title-row">
          <h1 className="cb-title">{product}</h1>
          <span className="cb-chip">{PLATFORM_LABEL[platform]}</span>
        </div>
        <p className="cb-tagline">{tagline}</p>
      </header>

      <main className="cb-main">
        <section className="cb-empty" aria-live="polite">
          <h2 className="cb-empty-title">Belum ada data</h2>
          <p className="cb-empty-desc">
            Pindai disk untuk melihat ruang yang bisa dibersihkan.
          </p>
        </section>
      </main>

      {version ? (
        <footer className="cb-footer">
          <span>Versi {version}</span>
        </footer>
      ) : null}
    </div>
  );
}
