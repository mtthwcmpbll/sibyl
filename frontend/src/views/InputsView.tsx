import { ErrorBanner } from "../components/ErrorBanner";
import type { Wizard, WizardState } from "../state/wizard";

/** Step 1: the two inputs — authoritative iconography rules + free-text deck style (US1). */
export function InputsView({ wizard, state }: { wizard: Wizard; state: WizardState }) {
  return (
    <section className="view inputs-view">
      <h2>Begin your deck</h2>
      <p className="muted">
        Describe the rules your suit iconography must follow, and the style you want to feel.
        Both are optional — leave them blank for tasteful defaults.
      </p>

      <label htmlFor="rules">Iconography rules (authoritative)</label>
      <textarea
        id="rules"
        data-testid="rules-input"
        rows={4}
        value={state.rules}
        placeholder="e.g. line art only, sacred geometry, no human faces…"
        onChange={(e) => wizard.setInputs(e.target.value, state.style)}
      />

      <label htmlFor="style">Deck style</label>
      <textarea
        id="style"
        data-testid="style-input"
        rows={4}
        value={state.style}
        placeholder="e.g. midnight indigo and gold, woodcut warmth…"
        onChange={(e) => wizard.setInputs(state.rules, e.target.value)}
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
