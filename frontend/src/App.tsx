import { useEffect, useMemo, useState } from "react";

import { getClient } from "./platform";
import { createWizard, type Wizard } from "./state/wizard";
import { useWizardState } from "./state/useWizard";
import { InputsView } from "./views/InputsView";
import { StyleChoiceView } from "./views/StyleChoiceView";
import { StyleGuideView } from "./views/StyleGuideView";
import { RevealView } from "./views/RevealView";
import { DoneView } from "./views/DoneView";

export function App() {
  const [wizard, setWizard] = useState<Wizard | null>(null);

  // Resolve the client (Tauri or mock) and load any existing active deck on open
  // (SC-003/004).
  useEffect(() => {
    let live = true;
    getClient().then(async (client) => {
      const w = createWizard(client);
      await w.loadActiveDeck();
      if (live) setWizard(w);
    });
    return () => {
      live = false;
    };
  }, []);

  if (!wizard) {
    return <main className="app loading">Preparing your studio…</main>;
  }
  return <WizardShell wizard={wizard} />;
}

function WizardShell({ wizard }: { wizard: Wizard }) {
  const state = useWizardState(wizard);

  const view = useMemo(() => {
    switch (state.step) {
      case "inputs":
        return <InputsView wizard={wizard} state={state} />;
      case "choosing":
        return <StyleChoiceView wizard={wizard} state={state} />;
      case "guide":
        return <StyleGuideView wizard={wizard} state={state} />;
      case "review":
        return <RevealView wizard={wizard} state={state} />;
      case "done":
        return <DoneView state={state} />;
    }
  }, [wizard, state]);

  return (
    <main className="app" data-step={state.step}>
      <header className="app__header">
        <h1>Sibyl</h1>
        <p className="tagline">a tarot deck made for you, because it is</p>
      </header>
      {view}
    </main>
  );
}
