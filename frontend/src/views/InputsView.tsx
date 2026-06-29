import { ErrorBanner } from "../components/ErrorBanner";
import type { Wizard, WizardState } from "../state/wizard";

/** Step 1: the two inputs — authoritative iconography rules + free-text deck style (US1). */
export function InputsView({ wizard, state }: { wizard: Wizard; state: WizardState }) {
  return (
    <section className="view inputs-view">
      <h2>Begin your deck</h2>
      <p className="muted">
        Describe the style you want your deck to feel like. The app handles the rules of good
        tarot iconography — you bring the vision. Leave it blank for a tasteful default.
      </p>

      <label htmlFor="style">Deck style</label>
      <textarea
        id="style"
        data-testid="style-input"
        rows={5}
        value={state.style}
        placeholder="e.g. midnight indigo and gold, woodcut warmth, art-nouveau linework…"
        onChange={(e) => wizard.setStyle(e.target.value)}
      />

      <ErrorBanner error={state.error} onRetry={() => wizard.generate()} onDismiss={() => wizard.clearError()} />

      <button
        type="button"
        className="primary"
        data-testid="generate"
        disabled={state.busy}
        onClick={() => wizard.generate()}
      >
        {state.busy ? "Conjuring…" : "Generate suit icons"}
      </button>
    </section>
  );
}
