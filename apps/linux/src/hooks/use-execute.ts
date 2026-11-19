import { useQueryClient } from "@tanstack/react-query";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  invokeCommand,
  subscribe,
  toIpcError,
  type IpcError,
} from "../lib/ipc";
import type {
  CleanFinished,
  CleanItemState,
  CleanProgress,
  CleanToken,
  DeleteMode,
  ExecuteCleanResult,
} from "../lib/types";
import { DISK_SUMMARY_KEY } from "./use-disk-summary";
import { HISTORY_KEY } from "./use-history";

export type ExecutePhase = "idle" | "running" | "done" | "error";

export type ItemProgress = {
  state: CleanItemState;
  freedBytes: number;
};

export type UseExecuteResult = {
  phase: ExecutePhase;
  actionId: string | null;
  items: Record<string, ItemProgress>;
  finished: CleanFinished | null;
  durationMs: number | null;
  error: IpcError | null;
  execute: (mode: DeleteMode) => Promise<void>;
};

/**
 * Eksekusi pembersihan: minta token, jalankan action, lalu ikuti event
 * `clean://progress` dan `clean://finished` milik action tersebut.
 */
export function useExecute(
  scanId: string,
  itemIds: string[],
): UseExecuteResult {
  const queryClient = useQueryClient();
  const actionIdRef = useRef<string | null>(null);
  const startedAtRef = useRef<number | null>(null);

  const [phase, setPhase] = useState<ExecutePhase>("idle");
  const [actionId, setActionId] = useState<string | null>(null);
  const [items, setItems] = useState<Record<string, ItemProgress>>({});
  const [finished, setFinished] = useState<CleanFinished | null>(null);
  const [durationMs, setDurationMs] = useState<number | null>(null);
  const [error, setError] = useState<IpcError | null>(null);

  useEffect(() => {
    const onProgress = (payload: CleanProgress) => {
      const current = actionIdRef.current;
      if (current && payload.action_id !== current) return;
      actionIdRef.current ??= payload.action_id;

      setItems((previous) => ({
        ...previous,
        [payload.item_id]: {
          state: payload.state,
          freedBytes: payload.freed_bytes,
        },
      }));
    };

    const onFinished = (payload: CleanFinished) => {
      const current = actionIdRef.current;
      if (current && payload.action_id !== current) return;
      actionIdRef.current ??= payload.action_id;

      setFinished(payload);
      setPhase("done");
      setDurationMs(
        startedAtRef.current === null
          ? null
          : Date.now() - startedAtRef.current,
      );
      void queryClient.invalidateQueries({ queryKey: DISK_SUMMARY_KEY });
      void queryClient.invalidateQueries({ queryKey: HISTORY_KEY });
      void queryClient.invalidateQueries({ queryKey: ["clean-plan"] });
    };

    const unsubscribeProgress = subscribe<CleanProgress>(
      "clean://progress",
      onProgress,
    );
    const unsubscribeFinished = subscribe<CleanFinished>(
      "clean://finished",
      onFinished,
    );

    return () => {
      unsubscribeProgress();
      unsubscribeFinished();
    };
  }, [queryClient]);

  const execute = useCallback(
    async (mode: DeleteMode) => {
      if (!scanId || itemIds.length === 0) return;

      setPhase("running");
      setError(null);
      setFinished(null);
      setItems({});
      setDurationMs(null);
      startedAtRef.current = Date.now();

      try {
        const { token } = await invokeCommand<CleanToken>(
          "request_clean_token",
          { scan_id: scanId, item_ids: itemIds },
        );

        const result = await invokeCommand<ExecuteCleanResult>(
          "execute_clean",
          {
            scan_id: scanId,
            item_ids: itemIds,
            mode,
            confirm_token: token,
          },
        );

        actionIdRef.current = result.action_id;
        setActionId(result.action_id);
      } catch (cause) {
        setError(toIpcError(cause));
        setPhase("error");
      }
    },
    [scanId, itemIds],
  );

  return { phase, actionId, items, finished, durationMs, error, execute };
}
