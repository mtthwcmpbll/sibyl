// Shared types mirroring the core's serialized shapes
// (see specs/001-deck-creation/contracts/). camelCase matches serde rename_all.

export type Suit = "cups" | "wands" | "swords" | "pentacles";
export type CourtRank = "page" | "knight" | "queen" | "king";
export interface CourtCardId {
  rank: CourtRank;
  suit: Suit;
}

export interface SuitIcon {
  suit: Suit;
  imageKey: string;
}

export interface StyleOption {
  id: string;
  label: string;
  suitIcons: SuitIcon[];
  promptUsed: string;
}

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface ShaderArea {
  id: string;
  rect: Rect;
  kind: string;
}

export interface FlourishRef {
  assetKey: string;
  rect: Rect;
  rotation: number;
}

export interface FrontLayout {
  aspect: number;
  suitIcon: Rect;
  cardImagery: Rect;
  title: Rect;
}

export interface CardStyleGuide {
  id: string;
  version: number;
  derivedFromStyleOptionId: string;
  promptUsed: string;
  borderChromeKey: string;
  cardBackKey: string;
  cardFrontLayout: FrontLayout;
  shaderAreas: ShaderArea[];
  flourishes: FlourishRef[];
}

export type Approval = "pending" | "approved" | "rejected";

export interface SampleCard {
  courtCard: CourtCardId;
  cardImageryKey: string;
  frontKey: string;
  backKey: string;
  approval: Approval;
}

export interface Deck {
  id: string;
  active: boolean;
  suitIcons: SuitIcon[];
  styleGuide: CardStyleGuide;
}

export interface IconStylesResult {
  sessionId: string;
  styleOptions: StyleOption[];
}

export interface RestartInputs {
  rules: string;
  style: string;
}

export interface AssetResponse {
  key: string;
  mime: string;
  /** A directly-usable data URL for the asset. */
  dataUrl: string;
}

/** Structured error matching the IPC contract's `{ code, message, retryable }`. */
export interface ClientError {
  code: string;
  message: string;
  retryable: boolean;
}

export function isClientError(e: unknown): e is ClientError {
  return (
    typeof e === "object" && e !== null && "code" in e && "retryable" in e
  );
}

export function courtCardSlug(c: CourtCardId): string {
  return `${c.rank}-of-${c.suit}`;
}
