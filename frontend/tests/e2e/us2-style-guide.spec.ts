import { expect, test } from "@playwright/test";

test("US2: a chosen style yields a style guide with the required elements", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("style-input").fill("indigo and gold");
  await page.getByTestId("generate").click();

  await page.locator('[data-testid^="style-option-"]').first().click();
  await page.getByTestId("continue-guide").click();

  const guide = page.getByTestId("style-guide-preview");
  await expect(guide).toBeVisible();
  // Designated animated-shader / UV areas are present (FR-008).
  await expect(page.getByTestId("shader-areas")).toContainText("uv-frame");
  // Proceed to compose the sample.
  await expect(page.getByTestId("compose-sample")).toBeVisible();
});
