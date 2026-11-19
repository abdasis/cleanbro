import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactElement,
  type ReactNode,
} from "react";
import { toIpcError } from "../ipc";
import { readStorage, writeStorage } from "../storage";
import {
  resolveErrorKey,
  translate,
  type Language,
  type TranslationKey,
  type TranslationParams,
} from "./dictionary";

const STORAGE_KEY = "sapu-bersih.language";

const LOCALE: Record<Language, string> = {
  id: "id-ID",
  en: "en-US",
};

export type I18nValue = {
  t: (key: TranslationKey, params?: TranslationParams) => string;
  locale: string;
  language: Language;
  setLanguage: (language: Language) => void;
};

const I18nContext = createContext<I18nValue | null>(null);

function readLanguage(): Language {
  return readStorage(STORAGE_KEY) === "en" ? "en" : "id";
}

export function I18nProvider({
  children,
}: {
  children: ReactNode;
}): ReactElement {
  const [language, setLanguageState] = useState<Language>(readLanguage);

  useEffect(() => {
    document.documentElement.lang = language;
    writeStorage(STORAGE_KEY, language);
  }, [language]);

  const setLanguage = useCallback((next: Language) => {
    setLanguageState(next);
  }, []);

  const t = useCallback(
    (key: TranslationKey, params?: TranslationParams) =>
      translate(language, key, params),
    [language],
  );

  const value = useMemo<I18nValue>(
    () => ({ t, locale: LOCALE[language], language, setLanguage }),
    [t, language, setLanguage],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nValue {
  const value = useContext(I18nContext);
  if (!value) {
    throw new Error("useI18n must be used inside I18nProvider");
  }
  return value;
}

/** Pesan error siap tampil: key kamus bila kode dikenali, selain itu pesan server. */
export function useErrorMessage(): (error: unknown) => string {
  const { t } = useI18n();

  return useCallback(
    (error: unknown) => {
      const ipcError = toIpcError(error);
      const key = resolveErrorKey(ipcError.code, ipcError.message);
      return key ? t(key) : ipcError.message;
    },
    [t],
  );
}
