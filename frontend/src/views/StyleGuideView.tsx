import { ErrorBanner } from "../components/ErrorBanner";
import { StyleGuidePreview } from "../components/StyleGuidePreview";
import type { Wizard, WizardState } from "../state/wizard";

/** Step 3: show the auto-generated card style guide, then compose the sample card (US2→US3). */
export function StyleGuideView({ wizard, state }: { wizard: Wizard; state: WizardState }) {
  return (
    <section className="view guide-view">
      <h2>Your card style guide</h2>
      <p className="muted">Borders, back, flourishes, and where animated shaders will live.</p>

      {state.guide && <StyleGuidePreview guide={state.guide} />}

      <ErrorBanner
        error={state.error}
        onRetry={() => wizard.composeSample()}
        onDismiss={() => wizard.clearError()}
      />

      <button
        type="button"
        className="primary"
        data-testid="compose-sample"
        disabled={state.busy}
        onClick={() => wizard.composeSample()}
      >
        {state.busy ? "Composing…" : "Reveal a card"}
      </button>
    </section>
  );
}
