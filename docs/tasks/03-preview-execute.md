# Task Breakdown: Preview, Eksekusi, Riwayat

> Bagian dari [TASKS.md](../TASKS.md).

## Milestone M3: Preview dan eksekusi

### Task 3.1: `preview_selection` dan `CleanPlan`

**Files:** `apps/linux/src-tauri/src/services/plan_service.rs`,
`apps/linux/src-tauri/src/commands/preview.rs`

**Test:** `plan_excludes_review_risk_by_default`,
`plan_recomputes_bytes_when_item_excluded`.

**Commit:** `feat: clean plan builder`.

### Task 3.2: Eksekusi mode trash dengan undo journal

**Files:** `apps/linux/src-tauri/src/services/exec_service.rs`,
`apps/linux/src-tauri/src/adapters/trash.rs`, `apps/linux/src-tauri/src/commands/execute.rs`

**Test:** `execute_moves_file_to_trash_and_records_journal`,
`execute_revalidates_guard_before_delete`,
`execute_skips_path_rejected_by_guard`.

**Implementasi:** dua fase (validasi ulang semua, lalu hapus), emit
`clean://progress` per item, tulis `action_items` + `undo_journal`.

**Commit:** `feat: trash execution with undo journal`.

### Task 3.3: Konfirmasi token untuk aksi > 5 GB

**Files:** `apps/linux/src-tauri/src/services/exec_service.rs`,
`apps/linux/src/lib/confirm.ts`, `apps/linux/src/features/execute/confirm-input.tsx`

**Test:** `confirm_token_required_above_threshold`,
`confirm_token_expires_after_five_minutes`,
`confirm_token_rejects_wrong_hash`.

**Commit:** `feat: destructive action confirmation gate`.

### Task 3.4: Riwayat dan undo

**Files:** `apps/linux/src-tauri/src/commands/history.rs`,
`apps/linux/src-tauri/src/store/action_repo.rs`, `apps/linux/src/features/history/history-table.tsx`

**Test:** `history_lists_newest_first`, `undo_restores_file_to_original_path`,
`undo_marks_expired_entry`.

**Commit:** `feat: history listing and undo`.

---

