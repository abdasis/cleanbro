import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type { RuleInfo } from "../lib/types";

export const RULES_KEY = ["rules"] as const;

/** Katalog rule tetap selama aplikasi berjalan. */
export function useRules() {
  return useQuery({
    queryKey: RULES_KEY,
    queryFn: () => invokeCommand<RuleInfo[]>("list_rules", {}),
    staleTime: Infinity,
  });
}
