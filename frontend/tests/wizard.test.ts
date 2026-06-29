import { beforeEach, describe, expect, it } from "vitest";

import { MockDeckForgeClient } from "../src/platform/mockClient";
import { createWizard } from "../src/state/wizard";
import { FAILING_RULES, FIXED_RULES, FIXED_STYLE } from "./fixtures";

function newWizard() {
  return createWizard(new MockDeckForgeClient());
}

beforeEach(() => {
  localStorage.clear();
});

describe("wizard state machine", () => {
  it("drives the full create→approve journey and persists the deck", async () => {
    const w = newWizard();
    w.setInputs(FIXED_RULES, FIXED_STYLE);

    await w.generate();
    let s = w.getState();
    expect(s.step).toBe("choosing");
    expect(s.options.length).toBeGreaterThanOrEqual(3); // SC-002

    await w.select(s.options[0].id);
    expect(w.getState().chosenId).toBe(s.options[0].id);

    await w.continueToGuide();
    expect(w.getState().step).toBe("guide");
    expect(w.getState().guide).toBeTruthy();

    await w.composeSample();
    s = w.getState();
    expect(s.step).toBe("review");
    expect(s.sample).toBeTruthy();

    await w.approve();
    s = w.getState();
    expect(s.step).toBe("done");
    expect(s.deck?.active).toBe(true);

    // Reopen → same active deck (SC-003/004).
    const w2 = newWizard();
    await w2.loadActiveDeck();
    expect(w2.getState().deck?.id).toBe(s.deck?.id);
  });

  it("rejecting returns to inputs, preserves inputs, and leaves no active deck (SC-005)", async () => {
    const w = newWizard();
    w.setInputs(FIXED_RULES, FIXED_STYLE);
    await w.generate();
    await w.select(w.getState().options[0].id);
    await w.continueToGuide();
    await w.composeSample();

    await w.reject();
    const s = w.getState();
    expect(s.step).toBe("inputs");
    expect(s.rules).toBe(FIXED_RULES);
    expect(s.sessionId).toBeUndefined();
    expect(await new MockDeckForgeClient().getActiveDeck()).toBeNull();
  });

  it("regenerate keeps inputs and advances the round (FR-006)", async () => {
    const w = newWizard();
    w.setInputs(FIXED_RULES, FIXED_STYLE);
    await w.generate();
    const firstId = w.getState().options[0].id;
    await w.regenerate();
    const s = w.getState();
    expect(s.rules).toBe(FIXED_RULES);
    expect(firstId.startsWith("r0-")).toBe(true);
    expect(s.options[0].id.startsWith("r1-")).toBe(true);
  });

  it("surfaces a provider failure as retryable, then succeeds on retry (FR-017/SC-006)", async () => {
    const w = newWizard();
    w.setInputs(FAILING_RULES, FIXED_STYLE);

    await w.generate();
    expect(w.getState().error?.retryable).toBe(true);
    expect(w.getState().step).toBe("inputs"); // inputs preserved, not advanced
    expect(w.getState().rules).toBe(FAILING_RULES);

    await w.generate(); // retry
    expect(w.getState().error).toBeUndefined();
    expect(w.getState().step).toBe("choosing");
  });
});
