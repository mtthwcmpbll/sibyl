import { useEffect, useState } from "react";

import { CardFlip } from "../components/CardFlip";
import { ErrorBanner } from "../components/ErrorBanner";
import { courtCardSlug } from "../platform";
import type { Wizard, WizardState } from "../state/wizard";

/** Step 4: present the court-card sample face down, flip to reveal, then approve/reject (US3). */
export function RevealView({ wizard, state }: { wizard: Wizard; state: WizardState }) {
  const [faceUp, setFaceUp] = useState(false);

  // Begin face down, then flip to reveal — the surprise moment (FR-010, SC-008).
  useEffect(() => {
    const t = setTimeout(() => setFaceUp(true), 700);
    return () => clearTimeout(t);
  }, []);

  if (!state.sample) return null;
  const title = courtCardSlug(state.sample.courtCard);

  return (
    <section className="view reveal-view">
      <h2>Your card</h2>

      <CardFlip
        frontKey={state.sample.frontKey}
        backKey={state.sample.backKey}
        faceUp={faceUp}
        title={title}
      />

      <p className="court-name" data-testid="court-name">
        {faceUp ? title.replace(/-/g, " ") : "…"}
      </p>

      <ErrorBanner
        error={state.error}
        onRetry={() => wizard.approve()}
        onDismiss={() => wizard.clearError()}
      />

      {faceUp && (
        <div className="actions" data-testid="approval-actions">
          <button type="button" data-testid="reject" disabled={state.busy} onClick={() => wizard.reject()}>
            Start over
          </button>
          <button
            type="button"
            className="primary"
            data-testid="approve"
            disabled={state.busy}
            onClick={() => wizard.approve()}
          >
            This is my deck
          </button>
        </div>
      )}
    </section>
  );
}
