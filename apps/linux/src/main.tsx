import { invoke } from "@tauri-apps/api/core";
import { AppShell } from "@cleanbro/ui";
import "@cleanbro/ui/styles.css";
import { StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

function App() {
  const [version, setVersion] = useState<string>();

  useEffect(() => {
    let active = true;

    invoke<{ version: string }>("app_info")
      .then((info) => {
        if (active) setVersion(info.version);
      })
      .catch(() => undefined);

    return () => {
      active = false;
    };
  }, []);

  return (
    <AppShell
      product="Sapu Bersih"
      platform="linux"
      tagline="Pindai, tinjau, lalu bersihkan ruang disk yang tidak terpakai."
      version={version}
    />
  );
}

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Elemen #root tidak ditemukan");
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
