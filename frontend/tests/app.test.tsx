import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect, it } from "vitest";

import { App } from "../src/App";

beforeEach(() => localStorage.clear());

it("walks the full deck-creation journey through the UI (US1→US3)", async () => {
  const user = userEvent.setup();
  render(<App />);

  // US1 — inputs → generate → choose
  await user.type(await screen.findByTestId("rules-input"), "geometric glyphs");
  await user.type(screen.getByTestId("style-input"), "indigo and gold");
  await user.click(screen.getByTestId("generate"));

  const grid = await screen.findByTestId("options-grid");
  const firstOption = grid.querySelector("button");
  expect(firstOption).toBeTruthy();
  await user.click(firstOption!);

  const cont = screen.getByTestId("continue-guide");
  await waitFor(() => expect(cont).toBeEnabled());
  await user.click(cont);

  // US2 — style guide
  await screen.findByTestId("style-guide-preview");
  expect(screen.getByTestId("shader-areas").textContent).toContain("uv-frame");
  await user.click(screen.getByTestId("compose-sample"));

  // US3 — sample begins face down, then flips before approval (SC-008)
  const card = await screen.findByTestId("sample-card");
  expect(card.getAttribute("data-faceup")).toBe("false");
  const approve = await screen.findByTestId("approve", {}, { timeout: 2000 });
  expect(card.getAttribute("data-faceup")).toBe("true");

  await user.click(approve);

  // Done — deck is active
  await screen.findByTestId("done-view");
  expect(screen.getByTestId("done-view")).toBeInTheDocument();
});
