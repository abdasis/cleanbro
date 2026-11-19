import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type { AppInfo } from "../lib/types";

export const APP_INFO_KEY = ["app-info"] as const;

export function useAppInfo() {
  return useQuery({
    queryKey: APP_INFO_KEY,
    queryFn: () => invokeCommand<AppInfo>("app_info", {}),
    staleTime: Infinity,
  });
}
