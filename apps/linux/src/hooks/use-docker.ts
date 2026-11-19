import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type {
  DockerPruneResult,
  DockerPruneTarget,
  DockerStatus,
} from "../lib/types";
import { DISK_SUMMARY_KEY } from "./use-disk-summary";

export const DOCKER_STATUS_KEY = ["docker-status"] as const;

/** Status Docker; di browser biasa query berakhir di state error. */
export function useDockerStatus() {
  return useQuery({
    queryKey: DOCKER_STATUS_KEY,
    queryFn: () => invokeCommand<DockerStatus>("docker_status", {}),
    staleTime: 30_000,
    refetchOnMount: true,
  });
}

/** Prune dua target aman; status Docker dan ringkasan disk ikut disegarkan. */
export function useDockerPrune() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (targets: DockerPruneTarget[]) =>
      invokeCommand<DockerPruneResult>("docker_prune", { targets }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: DOCKER_STATUS_KEY });
      void queryClient.invalidateQueries({ queryKey: DISK_SUMMARY_KEY });
    },
  });
}
