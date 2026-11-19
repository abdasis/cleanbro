import type { RiskLevel } from "./types";

const RULE_LABELS: Record<string, string> = {
  "cache.gradle": "Cache Gradle",
  "cache.bun": "Cache bun",
  "cache.npm": "Cache npm",
  "cache.pnpm": "Cache pnpm",
  "cache.pip": "Cache pip",
  "cache.uv": "Cache uv",
  "cache.composer": "Cache Composer",
  "cache.cargo.registry": "Cache Cargo",
  "cache.go": "Cache Go",
  "cache.cypress": "Cache Cypress",
  "cache.playwright": "Cache Playwright",
  "cache.huggingface": "Cache Hugging Face",
  "cache.dotslash": "Cache Dotslash",
  "cache.chrome": "Cache Chrome",
  "cache.thumbnails": "Thumbnail",
  "cache.tracker": "Cache Tracker",
  "build.node_modules": "node_modules",
  "build.rust_target": "Target Rust",
  "build.next": ".next",
  "build.dist": "Dist",
  "build.gradle": "Build Gradle",
  "build.flutter": "Build Flutter",
  "build.vendor": "Vendor Composer",
  "media.render_out": "Hasil render",
  "media.cache": "Cache media",
  "docker.containers_stopped": "Container berhenti",
  "docker.images_dangling": "Image dangling",
  "docker.images_unused": "Image tidak terpakai",
  "docker.volumes_unused": "Volume tidak terpakai",
  "system.apt_cache": "Cache apt",
  "system.journal": "Journal sistem",
  "system.old_kernels": "Kernel lama",
  "advice.big_files": "File besar",
  "advice.old_downloads": "Unduhan lama",
  "advice.trash": "Trash",
};

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

export const RISK_LABEL: Record<RiskLevel, string> = {
  safe: "aman",
  review: "perlu tinjauan",
  risky: "berisiko",
};

export type RuleMeta = {
  label: string;
  risk: RiskLevel;
};

/** Event scan hanya membawa `rule_id`; label dan risiko dipetakan di sini. */
export function ruleMeta(ruleId: string): RuleMeta {
  const risk: RiskLevel = RISKY_RULES.has(ruleId)
    ? "risky"
    : SAFE_RULES.has(ruleId) || ruleId.startsWith("cache.")
      ? "safe"
      : "review";

  return {
    label: RULE_LABELS[ruleId] ?? "Kategori lainnya",
    risk,
  };
}
