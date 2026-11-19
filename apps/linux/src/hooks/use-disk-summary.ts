import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type { DiskSummary } from "../lib/types";

export const DISK_SUMMARY_KEY = ["disk-summary"] as const;

export function useDiskSummary() {
  return useQuery({
    queryKey: DISK_SUMMARY_KEY,
    queryFn: () => invokeCommand<DiskSummary>("disk_summary", {}),
  });
}
