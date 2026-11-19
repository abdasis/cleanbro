import { useQueryClient } from "@tanstack/react-query";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  invokeCommand,
  subscribe,
  toIpcError,
  type IpcError,
} from "../lib/ipc";
import { DISK_SUMMARY_KEY } from "./use-disk-summary";
import type {
  CancelScanResult,
  CategoryResult,
  ScanCategoryEvent,
  ScanFinished,
  ScanMode,
  ScanProgress,
  StartScanResult,
} from "../lib/types";

export type ScanStatus = "idle" | "running" | "finished" | "error";

export type UseScanResult = {
  status: ScanStatus;
  scanId: string | null;
  progress: ScanProgress | null;
  categories: CategoryResult[];
  finished: ScanFinished | null;
  error: IpcError | null;
  pending: boolean;
  lastMode: ScanMode | null;
  start: (mode: ScanMode) => Promise<void>;
  cancel: () => Promise<void>;
};

/**
 * State scan hidup di hook ini. Event Rust di-merge ke state, lalu
 * ringkasan disk di-invalidasi saat scan selesai.
 *
 * `enabledRules` = daftar rule id aktif; `null` berarti semua rule (argumen
 * `categories` tidak dikirim).
 */
export function useScan(
  enabledRules: string[] | null = null,
): UseScanResult {
  const queryClient = useQueryClient();
  const scanIdRef = useRef<string | null>(null);
  const enabledRef = useRef(enabledRules);

  useEffect(() => {
    enabledRef.current = enabledRules;
  }, [enabledRules]);

  const [scanId, setScanId] = useState<string | null>(null);
  const [progress, setProgress] = useState<ScanProgress | null>(null);
  const [categories, setCategories] = useState<CategoryResult[]>([]);
  const [finished, setFinished] = useState<ScanFinished | null>(null);
  const [error, setError] = useState<IpcError | null>(null);
  const [pending, setPending] = useState(false);
  const [lastMode, setLastMode] = useState<ScanMode | null>(null);

  const updateScanId = useCallback((next: string | null) => {
    scanIdRef.current = next;
    setScanId(next);
  }, []);

  useEffect(() => {
    const unsubscribeProgress = subscribe<ScanProgress>(
      "scan://progress",
      (payload) => {
        if (payload.scan_id === scanIdRef.current) setProgress(payload);
      },
    );

    const unsubscribeCategory = subscribe<ScanCategoryEvent>(
      "scan://category",
      (payload) => {
        if (payload.scan_id !== scanIdRef.current) return;

        const result: CategoryResult = {
          rule_id: payload.rule_id,
          bytes: payload.bytes,
          item_count: payload.item_count,
        };

        setCategories((previous) => {
          const exists = previous.some(
            (item) => item.rule_id === result.rule_id,
          );
          return exists
            ? previous.map((item) =>
                item.rule_id === result.rule_id ? result : item,
              )
            : [...previous, result];
        });
      },
    );

    const unsubscribeFinished = subscribe<ScanFinished>(
      "scan://finished",
      (payload) => {
        if (payload.scan_id !== scanIdRef.current) return;
        setFinished(payload);
        void queryClient.invalidateQueries({ queryKey: DISK_SUMMARY_KEY });
      },
    );

    return () => {
      unsubscribeProgress();
      unsubscribeCategory();
      unsubscribeFinished();
    };
  }, [queryClient]);

  const start = useCallback(
    async (mode: ScanMode) => {
      setPending(true);
      setError(null);
      setProgress(null);
      setCategories([]);
      setFinished(null);
      setLastMode(mode);

      try {
        const categories = enabledRef.current;
        const result = await invokeCommand<StartScanResult>("start_scan", {
          mode,
          ...(categories === null ? {} : { categories }),
        });
        updateScanId(result.scan_id);
      } catch (cause) {
        setError(toIpcError(cause));
        updateScanId(null);
      } finally {
        setPending(false);
      }
    },
    [updateScanId],
  );

  const cancel = useCallback(async () => {
    const id = scanIdRef.current;
    if (!id) return;

    setPending(true);
    try {
      await invokeCommand<CancelScanResult>("cancel_scan", { scan_id: id });
    } catch (cause) {
      setError(toIpcError(cause));
    } finally {
      setPending(false);
    }
  }, []);

  const status: ScanStatus = scanId
    ? finished
      ? "finished"
      : "running"
    : error
      ? "error"
      : "idle";

  return {
    status,
    scanId,
    progress,
    categories,
    finished,
    error,
    pending,
    lastMode,
    start,
    cancel,
  };
}
