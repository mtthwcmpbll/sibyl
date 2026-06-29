import { expect, test } from "@playwright/test";

// Frontend half of T071 (FR-017, SC-006): a generation failure shows a retryable error and
// preserves the entered inputs; retry then succeeds.
test("a provider failure is retryable and preserves inputs", async ({ page }) => {
  await page.goto("/");
  // "boom" triggers a one-time retryable failure in the mock backend.
  await page.getByTestId("rules-input").fill("boom geometric glyphs");
  await page.getByTestId("style-input").fill("indigo and gold");
  await page.getByTestId("generate").click();

  await expect(page.getByTestId("error-banner")).toBeVisible();
  // Inputs are preserved.
  await expect(page.getByTestId("rules-input")).toHaveValue("boom geometric glyphs");

  // Retry succeeds.
  await page.getByTestId("retry").click();
  await expect(page.getByTestId("options-grid")).toBeVisible();
});
