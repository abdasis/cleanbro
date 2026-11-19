import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type { CleanPlan } from "../lib/types";

export function cleanPlanKey(scanId: string | null, ruleIds: string[]) {
  return ["clean-plan", scanId, ruleIds] as const;
}

export type SetExcludedVars = {
  itemId: string;
  excluded: boolean;
};

/**
 * Rencana pembersihan untuk satu kategori. Setiap perubahan pengecualian
 * memakai `CleanPlan` dari server agar total tidak dihitung di klien.
 */
export function useCleanPlan(scanId: string | null, ruleIds: string[]) {
  const queryClient = useQueryClient();
  const key = cleanPlanKey(scanId, ruleIds);

  const query = useQuery({
    queryKey: key,
    queryFn: () =>
      invokeCommand<CleanPlan>("preview_selection", {
        scan_id: scanId,
        rule_ids: ruleIds,
      }),
    enabled: Boolean(scanId) && ruleIds.length > 0,
  });

  const setExcluded = useMutation({
    mutationFn: ({ itemId, excluded }: SetExcludedVars) =>
      invokeCommand<CleanPlan>("set_item_excluded", {
        scan_id: scanId,
        item_id: itemId,
        excluded,
      }),
    onSuccess: () => {
      // Server mengembalikan plan seluruh scan, sedangkan sheet hanya
      // menampilkan satu kategori. Refetch dengan filter rule agar item
      // kategori lain tidak ikut tampil.
      void queryClient.invalidateQueries({ queryKey: key });
    },
  });

  return { query, setExcluded };
}
