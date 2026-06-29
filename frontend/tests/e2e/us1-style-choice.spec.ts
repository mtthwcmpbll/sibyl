import { expect, test } from "@playwright/test";

test("US1: generate, regenerate, and choose a suit-icon style", async ({ page }) => {
  await page.goto("/");

  await page.getByTestId("rules-input").fill("geometric glyphs, no faces");
  await page.getByTestId("style-input").fill("midnight indigo and gold");
  await page.getByTestId("generate").click();

  await expect(page.getByTestId("options-grid")).toBeVisible();
  const options = page.locator('[data-testid^="style-option-"]');
  await expect(options).toHaveCount(3); // SC-002 (>= 3; mock yields the default 3)

  // Regenerate keeps us on the choice step with a fresh round.
  await page.getByTestId("regenerate").click();
  await expect(page.getByTestId("options-grid")).toBeVisible();

  // Continue is gated on a selection.
  await expect(page.getByTestId("continue-guide")).toBeDisabled();
  await options.first().click();
  await expect(page.getByTestId("continue-guide")).toBeEnabled();
  await page.getByTestId("continue-guide").click();

  await expect(page.getByTestId("style-guide-preview")).toBeVisible();
});
