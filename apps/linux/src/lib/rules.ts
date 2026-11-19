import { ruleKey, type TranslationKey } from "./i18n/dictionary";
import type { RiskLevel } from "./types";

const SAFE_RULES = new Set([
  "build.node_modules",
  "build.rust_target",
  "build.next",
  "build.gradle",
  "build.flutter",
  "docker.containers_stopped",
  "docker.images_dangling",
  "system.apt_cache",
  "system.journal",
]);

const RISKY_RULES = new Set([
  "docker.volumes_unused",
  "system.old_kernels",
]);

/** Key kamus untuk label risiko; dipakai bersama oleh badge di semua layar. */
export const RISK_KEY: Record<RiskLevel, TranslationKey> = {
  safe: "risk.safe",
  review: "risk.review",
  risky: "risk.risky",
};

export type RuleMeta = {
  labelKey: TranslationKey;
  risk: RiskLevel;
};

/** Event scan hanya membawa `rule_id`; label dan risiko dipetakan di sini. */
export function ruleMeta(ruleId: string): RuleMeta {
  const risk: RiskLevel = RISKY_RULES.has(ruleId)
    ? "risky"
    : SAFE_RULES.has(ruleId) || ruleId.startsWith("cache.")
      ? "safe"
      : "review";

  return { labelKey: ruleKey(ruleId), risk };
}
