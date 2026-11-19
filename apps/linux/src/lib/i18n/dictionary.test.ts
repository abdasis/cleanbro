import { describe, expect, test } from "bun:test";
import {
  categoryKey,
  dictionaries,
  resolveErrorKey,
  ruleKey,
  translate,
} from "./dictionary";

describe("kamus i18n", () => {
  test("id dan en punya key yang sama", () => {
    expect(Object.keys(dictionaries.en).sort()).toEqual(
      Object.keys(dictionaries.id).sort(),
    );
  });

  test("interpolasi placeholder", () => {
    expect(translate("id", "preview.clean", { size: "34.1 GB" })).toBe(
      "Bersihkan (34.1 GB)",
    );
    expect(translate("en", "preview.clean", { size: "34.1 GB" })).toBe(
      "Clean (34.1 GB)",
    );
  });

  test("fallback rule dan kategori tak dikenal", () => {
    expect(ruleKey("cache.npm")).toBe("rule.cache.npm");
    expect(ruleKey("cache.tidak_ada")).toBe("common.ruleUnknown");
    expect(categoryKey("docker")).toBe("category.docker");
    expect(categoryKey("tidak-ada")).toBe("category.other");
  });

  test("kode error dikenali, pesan sentinel ikut diterjemahkan", () => {
    expect(resolveErrorKey("GuardRejected", "apa saja")).toBe(
      "error.GuardRejected",
    );
    expect(resolveErrorKey("TidakDikenal", "error.internal")).toBe(
      "error.internal",
    );
    expect(resolveErrorKey("TidakDikenal", "pesan server")).toBeNull();
  });
});
