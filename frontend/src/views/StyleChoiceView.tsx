import { ErrorBanner } from "../components/ErrorBanner";
import { StyleOption } from "../components/StyleOption";
import type { Wizard, WizardState } from "../state/wizard";

/** Step 2: review the generated style options and choose one; regenerate if none speak (US1). */
export function StyleChoiceView({ wizard, state }: { wizard: Wizard; state: WizardState }) {
  return (
    <section className="view choice-view">
      <h2>Which speaks to you?</h2>
      <p className="muted">Pick the suit-icon style that feels like your deck.</p>

      <div className="options-grid" data-testid="options-grid">
        {state.options.map((opt) => (
          <StyleOption
            key={opt.id}
            option={opt}
            selected={state.chosenId === opt.id}
            onSelect={(id) => wizard.select(id)}
          />
        ))}
      </div>

      <ErrorBanner
        error={state.error}
        onRetry={() => wizard.regenerate()}
        onDismiss={() => wizard.clearError()}
      />

      <div className="actions">
        <button type="button" data-testid="regenerate" disabled={state.busy} onClick={() => wizard.regenerate()}>
          Regenerate
        </button>
        <button
          type="button"
          className="primary"
          data-testid="continue-guide"
          disabled={state.busy || !state.chosenId}
          onClick={() => wizard.continueToGuide()}
        >
          Continue
        </button>
      </div>
    </section>
  );
}
