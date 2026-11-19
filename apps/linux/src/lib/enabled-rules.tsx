import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactElement,
  type ReactNode,
} from "react";
import { readStorage, writeStorage } from "./storage";

const STORAGE_KEY = "sapu-bersih.enabled-rules";

export type EnabledRulesValue = {
  /** `null` berarti semua rule aktif (default). */
  enabled: string[] | null;
  isEnabled: (ruleId: string) => boolean;
  toggle: (ruleId: string, allRuleIds: string[]) => void;
  selectAll: () => void;
  clear: () => void;
};

const EnabledRulesContext = createContext<EnabledRulesValue | null>(null);

function readEnabled(): string[] | null {
  const raw = readStorage(STORAGE_KEY);
  if (raw === null) return null;

  try {
    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) return null;
    const ids = parsed.filter(
      (value): value is string => typeof value === "string",
    );
    return ids.length === parsed.length ? ids : null;
  } catch {
    return null;
  }
}

function persist(next: string[] | null): void {
  writeStorage(STORAGE_KEY, next === null ? null : JSON.stringify(next));
}

export function EnabledRulesProvider({
  children,
}: {
  children: ReactNode;
}): ReactElement {
  const [enabled, setEnabled] = useState<string[] | null>(readEnabled);

  const isEnabled = useCallback(
    (ruleId: string) => enabled === null || enabled.includes(ruleId),
    [enabled],
  );

  const toggle = useCallback(
    (ruleId: string, allRuleIds: string[]) => {
      const active = enabled ?? allRuleIds;
      const next = active.includes(ruleId)
        ? active.filter((item) => item !== ruleId)
        : [...active, ruleId];
      const complete =
        allRuleIds.length > 0 && allRuleIds.every((item) => next.includes(item));
      const normalized = complete ? null : next;

      setEnabled(normalized);
      persist(normalized);
    },
    [enabled],
  );

  const selectAll = useCallback(() => {
    setEnabled(null);
    persist(null);
  }, []);

  const clear = useCallback(() => {
    setEnabled([]);
    persist([]);
  }, []);

  const value = useMemo<EnabledRulesValue>(
    () => ({ enabled, isEnabled, toggle, selectAll, clear }),
    [enabled, isEnabled, toggle, selectAll, clear],
  );

  return (
    <EnabledRulesContext.Provider value={value}>
      {children}
    </EnabledRulesContext.Provider>
  );
}

export function useEnabledRules(): EnabledRulesValue {
  const value = useContext(EnabledRulesContext);
  if (!value) {
    throw new Error("useEnabledRules must be used inside EnabledRulesProvider");
  }
  return value;
}
