import { QueryClientProvider } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@sapubersih/ui/styles.css";
import "./styles.css";
import { AppLayout } from "./app/app-layout";
import { queryClient } from "./lib/query";

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Elemen #root tidak ditemukan");
}

createRoot(rootElement).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <AppLayout />
    </QueryClientProvider>
  </StrictMode>,
);
