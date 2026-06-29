import type { DeckForgeClient } from "../platform/client";
import {
  isClientError,
  type CardStyleGuide,
  type ClientError,
  type Deck,
  type SampleCard,
  type StyleOption,
} from "../platform";

export type WizardStep = "inputs" | "choosing" | "guide" | "review" | "done";

export interface WizardState {
  step: WizardStep;
  sessionId?: string;
  rules: string;
  style: string;
  options: StyleOption[];
  chosenId?: string;
  guide?: CardStyleGuide;
  sample?: SampleCard;
  deck?: Deck;
  busy: boolean;
  error?: ClientError;
}

const initialState = (): WizardState => ({
  step: "inputs",
  rules: "",
  style: "",
  options: [],
  busy: false,
});

function toClientError(e: unknown): ClientError {
  if (isClientError(e)) return e;
  return { code: "unknown", message: String(e), retryable: false };
}

export interface Wizard {
  getState(): WizardState;
  subscribe(listener: () => void): () => void;
  setInputs(rules: string, style: string): void;
  clearError(): void;
  generate(): Promise<void>;
  regenerate(): Promise<void>;
  select(optionId: string): Promise<void>;
  continueToGuide(): Promise<void>;
  composeSample(): Promise<void>;
  approve(): Promise<void>;
  reject(): Promise<void>;
  loadActiveDeck(): Promise<void>;
}

/**
 * The wizard's logic, independent of React (so it is unit-testable with Vitest). Holds state,
 * orchestrates the client, and tracks busy/error for every async step (FR-016/017).
 */
export function createWizard(client: DeckForgeClient): Wizard {
  let state = initialState();
  const listeners = new Set<() => void>();

  const emit = () => listeners.forEach((l) => l());
  const set = (patch: Partial<WizardState>) => {
    state = { ...state, ...patch };
    emit();
  };

  async function run(fn: () => Promise<void>): Promise<void> {
    set({ busy: true, error: undefined });
    try {
      await fn();
    } catch (e) {
      set({ error: toClientError(e) });
    } finally {
      set({ busy: false });
    }
  }

  return {
    getState: () => state,
    subscribe(listener) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    setInputs(rules, style) {
      set({ rules, style });
    },
    clearError() {
      set({ error: undefined });
    },
    async generate() {
      await run(async () => {
        const r = await client.generateIconStyles({
          iconographyRules: state.rules,
          deckStyleText: state.style,
        });
        set({
          sessionId: r.sessionId,
          options: r.styleOptions,
          chosenId: undefined,
          step: "choosing",
        });
      });
    },
    async regenerate() {
      await run(async () => {
        const r = await client.generateIconStyles({
          sessionId: state.sessionId,
          iconographyRules: state.rules,
          deckStyleText: state.style,
        });
        set({ options: r.styleOptions, chosenId: undefined });
      });
    },
    async select(optionId) {
      await run(async () => {
        await client.selectIconStyle(state.sessionId!, optionId);
        set({ chosenId: optionId });
      });
    },
    async continueToGuide() {
      await run(async () => {
        const guide = await client.generateStyleGuide(state.sessionId!);
        set({ guide, step: "guide" });
      });
    },
    async composeSample() {
      await run(async () => {
        const sample = await client.composeSampleCard(state.sessionId!);
        set({ sample, step: "review" });
      });
    },
    async approve() {
      await run(async () => {
        const deck = await client.approveDeck(state.sessionId!);
        set({ deck, step: "done" });
      });
    },
    async reject() {
      await run(async () => {
        const inputs = await client.rejectAndRestart(state.sessionId!);
        set({
          step: "inputs",
          rules: inputs.rules,
          style: inputs.style,
          sessionId: undefined,
          options: [],
          chosenId: undefined,
          guide: undefined,
          sample: undefined,
        });
      });
    },
    async loadActiveDeck() {
      const deck = await client.getActiveDeck();
      if (deck) set({ deck });
    },
  };
}
