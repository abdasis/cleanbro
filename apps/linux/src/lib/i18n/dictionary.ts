import { en } from "./messages-en";
import { id } from "./messages-id";

export type Language = "id" | "en";
export type TranslationKey = keyof typeof id;
export type TranslationParams = Record<string, string | number>;

export const dictionaries: Record<Language, Record<TranslationKey, string>> = {
  id,
  en,
};

const PLACEHOLDER = /\{(\w+)\}/g;

/** Terjemahkan key; placeholder `{name}` diganti dari `params`. */
export function translate(
  language: Language,
  key: TranslationKey,
  params?: TranslationParams,
): string {
  const template = dictionaries[language][key];

  if (!params) return template;

  return template.replace(PLACEHOLDER, (match, name: string) => {
    const value = params[name];
    return value === undefined ? match : String(value);
  });
}

function hasKey(key: string): key is TranslationKey {
  return Object.prototype.hasOwnProperty.call(id, key);
}

/** Key label rule dari id backend, contoh `cache.gradle`. */
export function ruleKey(ruleId: string): TranslationKey {
  const key = `rule.${ruleId}`;
  return hasKey(key) ? key : "common.ruleUnknown";
}

/** Key label kategori dari slug backend, contoh `developer-cache`. */
export function categoryKey(categorySlug: string): TranslationKey {
  const key = `category.${categorySlug}`;
  return hasKey(key) ? key : "category.other";
}

/**
 * Kode error stabil backend dipetakan ke key kamus. Pesan sentinel
 * (`error.*`) dari lapisan IPC juga dikenali; selain itu pakai pesan server.
 */
export function resolveErrorKey(
  code: string,
  message: string,
): TranslationKey | null {
  const byCode = `error.${code}`;
  if (hasKey(byCode)) return byCode;

  if (message.startsWith("error.") && hasKey(message)) return message;

  return null;
}
