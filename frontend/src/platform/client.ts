import type {
  AssetResponse,
  CardStyleGuide,
  Deck,
  IconStylesResult,
  RestartInputs,
  SampleCard,
} from "./types";

export interface GenerateIconStylesInput {
  sessionId?: string;
  iconographyRules: string;
  deckStyleText: string;
  count?: number;
}

/**
 * The single boundary between the frontend and the core (constitution Principle II +
 * frontend-portability). The desktop app implements this via Tauri `invoke`; tests and the
 * plain-web build implement it with the in-memory mock. No component touches Tauri directly.
 */
export interface DeckForgeClient {
  generateIconStyles(input: GenerateIconStylesInput): Promise<IconStylesResult>;
  selectIconStyle(sessionId: string, styleOptionId: string): Promise<void>;
  generateStyleGuide(sessionId: string): Promise<CardStyleGuide>;
  composeSampleCard(sessionId: string): Promise<SampleCard>;
  getAsset(key: string): Promise<AssetResponse>;
  approveDeck(sessionId: string): Promise<Deck>;
  rejectAndRestart(sessionId: string): Promise<RestartInputs>;
  getActiveDeck(): Promise<Deck | null>;
}
