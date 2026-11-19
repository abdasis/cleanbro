import type { ReactElement } from "react";
import { ScanLine, ScanSearch } from "lucide-react";
import { Button } from "../../components/ui/button";
import type { ScanMode } from "../../lib/types";

export type ScanActionsProps = {
  disabled: boolean;
  onStart: (mode: ScanMode) => void;
};

const ICON_PROPS = { size: 16, strokeWidth: 1.5 } as const;

export function ScanActions({
  disabled,
  onStart,
}: ScanActionsProps): ReactElement {
  return (
    <div className="flex items-center gap-2">
      <Button
        variant="primary"
        disabled={disabled}
        onClick={() => onStart("quick")}
      >
        <ScanLine {...ICON_PROPS} aria-hidden="true" />
        Scan cepat
      </Button>
      <Button
        variant="outline"
        disabled={disabled}
        onClick={() => onStart("deep")}
      >
        <ScanSearch {...ICON_PROPS} aria-hidden="true" />
        Scan dalam
      </Button>
    </div>
  );
}
