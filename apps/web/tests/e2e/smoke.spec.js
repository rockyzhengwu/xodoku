import { expect, test } from "@playwright/test";

test("player layout fits the viewport and generation dialog opens", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "Xodoku" })).toBeVisible();
  const panel = page.getByTestId("player-control-panel");
  await expect(panel).toBeVisible();
  const layout = await panel.evaluate((element) => ({
    panelWidth: element.getBoundingClientRect().width,
    viewportWidth: window.innerWidth,
  }));
  if (layout.viewportWidth >= 900) {
    expect(layout.panelWidth).toBeLessThanOrEqual(320);
  }
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.getByRole("dialog", { name: "Generate Sudoku" })).toBeVisible();
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth);
  expect(overflow).toBe(false);
});

test("techniques directory opens an article", async ({ page }) => {
  await page.goto("/techniques");
  await expect(page.getByRole("heading", { name: "Solving techniques" })).toBeVisible();
  await expect(page.getByText("Reserved").first()).toBeVisible();
  await expect(page.getByText("Alias").first()).toBeVisible();
  await page.goto("/techniques/singles");
  await expect(page.locator(".article-content")).toBeVisible();
});

test("sue de coq technique article is linked", async ({ page }) => {
  await page.goto("/techniques");
  await page.getByRole("link", { name: "Sue de Coq" }).last().click();
  await expect(page).toHaveURL(/\/techniques\/sue-de-coq$/);
  await expect(page.locator(".article-content")).toContainText("Extended Sue de Coq");
});

test("scanner upload screen fits the viewport", async ({ page }) => {
  await page.goto("/scanner");
  await expect(page.getByRole("heading", { name: "Sudoku Scanner" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Paste from clipboard" })).toBeVisible();
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth);
  expect(overflow).toBe(false);
});

test("keyboard input works for the first cell", async ({ page }) => {
  const puzzle = "..2...3...3......767............61..........47..52..6..2.3.49...9...7.8...69.8.1.";
  await page.goto(`/play?s=${encodeURIComponent(puzzle)}`);
  await page.waitForURL("/");
  const firstCellCover = page.locator('[data-index="0"] > rect').last();
  await firstCellCover.click();
  await expect(firstCellCover).toHaveClass(/stroke-\[var\(--cell-selected-stroke-color\)\]/);
  await page.keyboard.press("1");
  await expect(page.locator('[data-index="0"] [data-kind="digit"]')).toHaveText("1");
});
