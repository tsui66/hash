import { test, expect } from "@playwright/test";

test("guest user navigation to login and signup pages", async ({ page }) => {
  await page.goto("/");
  await page.waitForNavigation({ url: "**/login" });

  await expect(page.locator("text=Sign in to your account")).toBeVisible();
  await expect(page.locator("text=No account? No problem")).toBeVisible();

  await page.click("text=No account? No problem");

  await Promise.all([
    page.click("text=Create a free account"),
    page.waitForNavigation({ url: "**/signup" }),
  ]);

  await expect(
    page.locator('[placeholder="Enter your email address.."]'),
  ).toBeVisible();

  await expect(
    page.locator('button:has-text("Continue with email")'),
  ).toBeVisible();

  await expect(
    page.locator(
      "text=Alternatively if you already have a HASH account, Click here to log in",
    ),
  ).toBeVisible();

  await Promise.all([
    page.click("text=Click here to log in"),
    page.waitForNavigation({ url: "**/login" }),
  ]);

  await expect(
    page.locator('h1:has-text("Sign in to your account")'),
  ).toBeVisible();

  await page.click('[placeholder="Enter your email or shortname"]');

  await page.fill(
    '[placeholder="Enter your email or shortname"]',
    "hello world",
  );

  await page.click('button:has-text("Submit")');

  await page.click(
    "text=A user with the shortname 'hello world' could not be found.",
  );
});

test("guest user navigation to inaccessible pages", async ({ page }) => {
  await page.goto("/non/existing/page");
  await expect(page).toHaveTitle("404: This page could not be found");

  await expect(
    page.locator("text=This page could not be found."),
  ).toBeVisible();

  await expect(page.locator("button")).toHaveText(["Sign up", "Sign in"]);
});
