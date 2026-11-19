export type WarnLevel = "ok" | "warn" | "critical";

export type ScanMode = "quick" | "deep";

export type RiskLevel = "safe" | "review" | "risky";

export type DeleteMode = "trash" | "delete";

export type CleanItemState =
  | "pending"
  | "running"
  | "done"
  | "failed"
  | "skipped";

export interface DiskSummary {
  total: number;
  used: number;
  free: number;
  percent: number;
  mount: string;
  warn_level: WarnLevel;
}

export interface CategoryResult {
  rule_id: string;
  bytes: number;
  item_count: number;
}

export interface ScanProgress {
  scan_id: string;
  scanned_files: number;
  current_path: string;
  categories_done: number;
}

export interface ScanFinished {
  scan_id: string;
  partial: boolean;
  duration_ms: number;
}

/** Payload event `scan://category` = hasil kategori + id scan. */
export interface ScanCategoryEvent extends CategoryResult {
  scan_id: string;
}

export interface StartScanResult {
  scan_id: string;
}

export interface CancelScanResult {
  cancelled: boolean;
}

/** Satu item pada rencana pembersihan hasil preview. */
export interface PlanItem {
  item_id: string;
  rule_id: string;
  path: string;
  bytes: number;
  item_count: number;
  risk: RiskLevel;
  excluded: boolean;
  reason: string;
}

/** Rencana pembersihan; total sudah dihitung server setelah pengecualian. */
export interface CleanPlan {
  scan_id: string;
  total_bytes: number;
  total_items: number;
  requires_confirmation: boolean;
  items: PlanItem[];
}

export interface CleanToken {
  token: string;
  expires_at: number;
}

export interface ExecuteCleanResult {
  action_id: string;
}

/** Payload event `clean://progress`, satu entri per item. */
export interface CleanProgress {
  action_id: string;
  item_id: string;
  state: CleanItemState;
  freed_bytes: number;
}

/** Payload event `clean://finished`. */
export interface CleanFinished {
  action_id: string;
  deleted: number;
  failed: number;
  skipped: number;
  freed_bytes: number;
}

export interface ActionSummary {
  id: string;
  scan_id: string;
  mode: DeleteMode;
  started_at: number;
  finished_at: number | null;
  freed_bytes: number;
  status: string;
  item_count: number;
  undoable: boolean;
}

export interface HistoryPage {
  items: ActionSummary[];
  total: number;
}

export interface UndoResult {
  restored: number;
  failed: number;
}

/** Ringkasan `docker system df`. */
export interface DockerDf {
  images_total: number;
  images_active: number;
  containers_total: number;
  containers_active: number;
  volumes_total: number;
  local_volumes: number;
  reclaimable_bytes: number;
}

export interface DockerContainer {
  id: string;
  name: string;
  image: string;
  state: string;
  size_bytes: number;
}

export interface DockerImage {
  id: string;
  repository: string;
  tag: string;
  size_bytes: number;
  in_use: boolean;
}

export interface DockerStatus {
  available: boolean;
  reason: string | null;
  df: DockerDf | null;
  containers: DockerContainer[];
  images: DockerImage[];
}

/** Hanya dua target aman yang ditawarkan UI. */
export type DockerPruneTarget = "containers" | "images_dangling";

export interface DockerPruneResult {
  freed_bytes: number;
  deleted: number;
  output: string;
}
