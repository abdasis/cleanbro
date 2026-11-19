import {
  useInfiniteQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invokeCommand } from "../lib/ipc";
import type { ActionSummary, HistoryPage, UndoResult } from "../lib/types";
import { DISK_SUMMARY_KEY } from "./use-disk-summary";

export const HISTORY_KEY = ["history"] as const;
export const HISTORY_PAGE_SIZE = 20;

export type UseHistoryResult = {
  items: ActionSummary[];
  total: number;
  hasMore: boolean;
  isPending: boolean;
  isError: boolean;
  error: unknown;
  isFetchingNextPage: boolean;
  refetch: () => void;
  loadMore: () => void;
};

/** Daftar aksi pembersihan dengan pagination limit 20 per halaman. */
export function useHistory(limit = HISTORY_PAGE_SIZE): UseHistoryResult {
  const query = useInfiniteQuery({
    queryKey: [...HISTORY_KEY, limit],
    queryFn: ({ pageParam }) =>
      invokeCommand<HistoryPage>("list_history", {
        limit,
        offset: pageParam,
      }),
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      const loaded = allPages.reduce(
        (sum, page) => sum + page.items.length,
        0,
      );
      return loaded < lastPage.total ? loaded : undefined;
    },
  });

  const pages = query.data?.pages ?? [];
  const items = pages.flatMap((page) => page.items);
  const total = pages[0]?.total ?? 0;

  return {
    items,
    total,
    hasMore: query.hasNextPage,
    isPending: query.isPending,
    isError: query.isError,
    error: query.error,
    isFetchingNextPage: query.isFetchingNextPage,
    refetch: () => void query.refetch(),
    loadMore: () => {
      if (query.hasNextPage && !query.isFetchingNextPage) {
        void query.fetchNextPage();
      }
    },
  };
}

/** Undo dipakai di halaman riwayat dan di layar hasil eksekusi. */
export function useUndoAction() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (actionId: string) =>
      invokeCommand<UndoResult>("undo_action", { action_id: actionId }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: HISTORY_KEY });
      void queryClient.invalidateQueries({ queryKey: DISK_SUMMARY_KEY });
    },
  });
}
