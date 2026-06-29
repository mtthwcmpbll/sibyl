import { expect, test } from "@playwright/test";

async function driveToReveal(page: import("@playwright/test").Page) {
  await page.goto("/");
  await page.getByTestId("rules-input").fill("geometric glyphs");
  await page.getByTestId("style-input").fill("indigo and gold");
  await page.getByTestId("generate").click();
  await page.locator('[data-testid^="style-option-"]').first().click();
  await page.getByTestId("continue-guide").click();
  await page.getByTestId("compose-sample").click();
}

test("US3: sample begins face down, flips, and approval persists the deck", async ({ page }) => {
  await driveToReveal(page);

  const card = page.getByTestId("sample-card");
  await expect(card).toBeVisible();
  // Begins face down (SC-008), then flips to reveal.
  await expect(card).toHaveAttribute("data-faceup", "false");
  await expect(page.getByTestId("approve")).toBeVisible();
  await expect(card).toHaveAttribute("data-faceup", "true");

  await page.getByTestId("approve").click();
  await expect(page.getByTestId("done-view")).toBeVisible();

  // The approved deck persists across a reload (SC-003/004).
  const stored = await page.evaluate(() => localStorage.getItem("deckforge.activeDeck"));
  expect(stored).toBeTruthy();
});

test("US3: rejecting returns to inputs with inputs preserved and no active deck", async ({
  page,
}) => {
  await driveToReveal(page);
  await page.getByTestId("reject").click();

  await expect(page.getByTestId("generate")).toBeVisible();
  await expect(page.getByTestId("rules-input")).toHaveValue("geometric glyphs");
  const stored = await page.evaluate(() => localStorage.getItem("deckforge.activeDeck"));
  expect(stored).toBeNull();
});
