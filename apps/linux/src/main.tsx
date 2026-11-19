import { QueryClientProvider } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@sapubersih/ui/styles.css";
import "./styles.css";
import { AppLayout } from "./app/app-layout";
import { EnabledRulesProvider } from "./lib/enabled-rules";
import { I18nProvider } from "./lib/i18n/i18n-context";
import { queryClient } from "./lib/query";

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Elemen #root tidak ditemukan");
}

createRoot(rootElement).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <I18nProvider>
        <EnabledRulesProvider>
          <AppLayout />
        </EnabledRulesProvider>
      </I18nProvider>
    </QueryClientProvider>
  </StrictMode>,
);
