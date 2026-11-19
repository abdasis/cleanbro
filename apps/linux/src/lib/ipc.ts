import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type CommandName =
  | "disk_summary"
  | "start_scan"
  | "cancel_scan"
  | "preview_selection"
  | "set_item_excluded"
  | "request_clean_token"
  | "execute_clean"
  | "list_history"
  | "undo_action"
  | "app_info"
  | "docker_status"
  | "docker_prune";

export type EventName =
  | "scan://progress"
  | "scan://category"
  | "scan://finished"
  | "clean://progress"
  | "clean://finished";

/** Bentuk error IPC yang stabil dari Rust: `{ code, message, detail }`. */
export class IpcError extends Error {
  readonly code: string;
  readonly detail?: string;

  constructor(code: string, message: string, detail?: string) {
    super(message);
    this.name = "IpcError";
    this.code = code;
    this.detail = detail;
  }
}

export function toIpcError(cause: unknown): IpcError {
  if (cause instanceof IpcError) return cause;

  if (typeof cause === "object" && cause !== null) {
    const { code, message, detail } = cause as {
      code?: unknown;
      message?: unknown;
      detail?: unknown;
    };

    if (typeof message === "string") {
      return new IpcError(
        typeof code === "string" ? code : "Internal",
        message,
        typeof detail === "string" ? detail : undefined,
      );
    }
  }

  if (typeof cause === "string") return new IpcError("Internal", cause);

  return new IpcError("Internal", "Perintah gagal dijalankan. Coba lagi ya.");
}

export function isTauri(): boolean {
  return (
    typeof window !== "undefined" &&
    "__TAURI_INTERNALS__" in window &&
    window.__TAURI_INTERNALS__ !== undefined
  );
}

/**
 * Panggil command Tauri. Di browser biasa selalu menolak dengan `IpcError`
 * agar pemanggil bisa menampilkan state error netral.
 */
export async function invokeCommand<T>(
  command: CommandName,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isTauri()) {
    throw new IpcError(
      "Unavailable",
      "Aplikasi desktop belum tersedia di lingkungan ini.",
    );
  }

  try {
    return await invoke<T>(command, args);
  } catch (cause) {
    throw toIpcError(cause);
  }
}

/** Langganan event; di browser biasa mengembalikan no-op unsubscribe. */
export function subscribe<T>(
  event: EventName,
  handler: (payload: T) => void,
): () => void {
  if (!isTauri()) return () => {};

  let active = true;
  let unlisten: UnlistenFn | undefined;

  listen<T>(event, (incoming) => {
    if (active) handler(incoming.payload);
  })
    .then((fn) => {
      if (active) unlisten = fn;
      else fn();
    })
    .catch(() => undefined);

  return () => {
    active = false;
    unlisten?.();
  };
}

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}
