import type { SibylClient } from "./client";
import { MockSibylClient } from "./mockClient";

export type { SibylClient } from "./client";
export * from "./types";

/** True when running inside the Tauri desktop shell (vs. a plain browser/Storybook/Playwright). */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

let cached: SibylClient | null = null;

/**
 * Resolve the active client. In the desktop shell this lazily loads the Tauri implementation;
 * everywhere else (tests, Storybook, plain web) it uses the deterministic mock. The dynamic
 * import keeps `@tauri-apps/api` out of the browser bundle entirely.
 */
export async function getClient(): Promise<SibylClient> {
  if (cached) return cached;
  if (isTauri()) {
    const { TauriSibylClient } = await import("./tauriClient");
    cached = new TauriSibylClient();
  } else {
    cached = new MockSibylClient();
  }
  return cached;
}

/** Override the client (used by tests to inject a fresh mock). */
export function setClient(client: SibylClient): void {
  cached = client;
}
