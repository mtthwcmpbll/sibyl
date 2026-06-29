import { beforeEach, describe, expect, it } from "vitest";

import { MockSibylClient } from "../src/platform/mockClient";
import { courtCardSlug, type CourtRank } from "../src/platform";
import { ALL_SUITS, FIXED_STYLE } from "./fixtures";

const RANKS: CourtRank[] = ["page", "knight", "queen", "king"];

beforeEach(() => localStorage.clear());

async function driveToSample(c: MockSibylClient) {
  const r = await c.generateIconStyles({
    deckStyleText: FIXED_STYLE,
  });
  await c.selectIconStyle(r.sessionId, r.styleOptions[0].id);
  await c.generateStyleGuide(r.sessionId);
  const sample = await c.composeSampleCard(r.sessionId);
  return { sessionId: r.sessionId, sample };
}

describe("MockSibylClient contract", () => {
  it("returns at least three options, each covering every suit", async () => {
    const c = new MockSibylClient();
    const r = await c.generateIconStyles({
      deckStyleText: FIXED_STYLE,
    });
    expect(r.styleOptions.length).toBeGreaterThanOrEqual(3);
    for (const o of r.styleOptions) {
      expect(o.suitIcons.map((i) => i.suit).sort()).toEqual(ALL_SUITS);
    }
  });

  it("is deterministic in session id for the same inputs", async () => {
    const a = await new MockSibylClient().generateIconStyles({
      deckStyleText: FIXED_STYLE,
    });
    const b = await new MockSibylClient().generateIconStyles({
      deckStyleText: FIXED_STYLE,
    });
    expect(b.sessionId).toBe(a.sessionId);
  });

  it("composes a sample that is always a court card", async () => {
    const c = new MockSibylClient();
    const { sample } = await driveToSample(c);
    expect(RANKS).toContain(sample.courtCard.rank);
    expect(ALL_SUITS).toContain(sample.courtCard.suit);
    expect(sample.approval).toBe("pending");
    expect(courtCardSlug(sample.courtCard)).toMatch(/-of-/);
  });

  it("approve persists the active deck; reject does not", async () => {
    const c = new MockSibylClient();
    const { sessionId } = await driveToSample(c);
    const deck = await c.approveDeck(sessionId);
    expect(deck.active).toBe(true);
    expect((await c.getActiveDeck())?.id).toBe(deck.id);
  });

  it("rejects unknown sessions and missing-selection errors with contract codes", async () => {
    const c = new MockSibylClient();
    await expect(c.generateStyleGuide("nope")).rejects.toMatchObject({
      code: "unknown_session",
    });
    const r = await c.generateIconStyles({
      deckStyleText: FIXED_STYLE,
    });
    await expect(c.generateStyleGuide(r.sessionId)).rejects.toMatchObject({
      code: "no_style_selected",
    });
  });
});
