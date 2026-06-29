import { invoke } from "@tauri-apps/api/core";

import type { DeckForgeClient, GenerateIconStylesInput } from "./client";
import type {
  AssetResponse,
  CardStyleGuide,
  Deck,
  IconStylesResult,
  RestartInputs,
  SampleCard,
} from "./types";

/**
 * Desktop implementation of the client: every method is a single Tauri `invoke`. This is the
 * ONLY module that imports Tauri APIs, keeping every component portable (Principle II).
 */
export class TauriDeckForgeClient implements DeckForgeClient {
  generateIconStyles(input: GenerateIconStylesInput): Promise<IconStylesResult> {
    return invoke("generate_icon_styles", {
      sessionId: input.sessionId ?? null,
      iconographyRules: input.iconographyRules,
      deckStyleText: input.deckStyleText,
      count: input.count ?? null,
    });
  }

  selectIconStyle(sessionId: string, styleOptionId: string): Promise<void> {
    return invoke("select_icon_style", { sessionId, styleOptionId });
  }

  generateStyleGuide(sessionId: string): Promise<CardStyleGuide> {
    return invoke("generate_style_guide", { sessionId });
  }

  composeSampleCard(sessionId: string): Promise<SampleCard> {
    return invoke("compose_sample_card", { sessionId });
  }

  async getAsset(key: string): Promise<AssetResponse> {
    const res = await invoke<{ key: string; mime: string; base64: string }>("get_asset", {
      key,
    });
    return {
      key: res.key,
      mime: res.mime,
      dataUrl: `data:${res.mime};base64,${res.base64}`,
    };
  }

  approveDeck(sessionId: string): Promise<Deck> {
    return invoke("approve_deck", { sessionId });
  }

  rejectAndRestart(sessionId: string): Promise<RestartInputs> {
    return invoke("reject_and_restart", { sessionId });
  }

  getActiveDeck(): Promise<Deck | null> {
    return invoke("get_active_deck");
  }
}
