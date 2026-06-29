import { useSyncExternalStore } from "react";

import type { Wizard, WizardState } from "./wizard";

/** Subscribe a React component to a wizard store. */
export function useWizardState(wizard: Wizard): WizardState {
  return useSyncExternalStore(wizard.subscribe, wizard.getState, wizard.getState);
}
