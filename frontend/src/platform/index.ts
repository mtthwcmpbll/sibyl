import type { DeckForgeClient } from "./client";
import { MockDeckForgeClient } from "./mockClient";

export type { DeckForgeClient } from "./client";
export * from "./types";

/** True when running inside the Tauri desktop shell (vs. a plain browser/Storybook/Playwright). */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

let cached: DeckForgeClient | null = null;

/**
 * Resolve the active client. In the desktop shell this lazily loads the Tauri implementation;
 * everywhere else (tests, Storybook, plain web) it uses the deterministic mock. The dynamic
 * import keeps `@tauri-apps/api` out of the browser bundle entirely.
 */
export async function getClient(): Promise<DeckForgeClient> {
  if (cached) return cached;
  if (isTauri()) {
    const { TauriDeckForgeClient } = await import("./tauriClient");
    cached = new TauriDeckForgeClient();
  } else {
    cached = new MockDeckForgeClient();
  }
  return cached;
}

/** Override the client (used by tests to inject a fresh mock). */
export function setClient(client: DeckForgeClient): void {
  cached = client;
}
