import type { DeckForgeClient, GenerateIconStylesInput } from "./client";
import type {
  AssetResponse,
  CardStyleGuide,
  CourtCardId,
  CourtRank,
  Deck,
  IconStylesResult,
  RestartInputs,
  SampleCard,
  StyleOption,
  Suit,
} from "./types";

const SUITS: Suit[] = ["cups", "wands", "swords", "pentacles"];
const RANKS: CourtRank[] = ["page", "knight", "queen", "king"];
const ACTIVE_KEY = "deckforge.activeDeck";

function fnv1a(s: string): number {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

function err(code: string, message: string, retryable = false): never {
  throw { code, message, retryable };
}

function allCourtCards(): CourtCardId[] {
  const out: CourtCardId[] = [];
  for (const rank of RANKS) for (const suit of SUITS) out.push({ rank, suit });
  return out;
}

interface MockSession {
  id: string;
  style: string;
  round: number;
  options: StyleOption[];
  chosenId?: string;
  guide?: CardStyleGuide;
  sample?: SampleCard;
}

/**
 * Deterministic, in-memory implementation of the core contract. Lets the entire frontend run
 * — in Storybook, Playwright, and a plain web build — with no Tauri and no network. Mirrors
 * the backend's semantics closely enough to drive every wizard path.
 */
export class MockDeckForgeClient implements DeckForgeClient {
  private sessions = new Map<string, MockSession>();
  private attempts = new Map<string, number>();

  async generateIconStyles(input: GenerateIconStylesInput): Promise<IconStylesResult> {
    // Failure-injection hook for the failure-path test (FR-017): a deck style containing
    // "boom" fails retryably on the first attempt, then succeeds.
    if (/boom/i.test(input.deckStyleText)) {
      const n = (this.attempts.get(input.deckStyleText) ?? 0) + 1;
      this.attempts.set(input.deckStyleText, n);
      if (n === 1) err("provider_failed", "image backend is down", true);
    }

    let session = input.sessionId ? this.sessions.get(input.sessionId) : undefined;
    if (session) {
      session.round += 1;
    } else {
      const id = `sess-${fnv1a(input.deckStyleText).toString(16)}`;
      session = {
        id,
        style: input.deckStyleText,
        round: 0,
        options: [],
      };
      this.sessions.set(id, session);
    }

    const count = Math.max(input.count ?? 3, 3);
    const options: StyleOption[] = [];
    for (let o = 0; o < count; o++) {
      const optionId = `r${session.round}-opt${o}`;
      options.push({
        id: optionId,
        label: `Option ${String.fromCharCode(65 + o)}`,
        suitIcons: SUITS.map((suit) => ({
          suit,
          imageKey: `mock/${session!.id}/${optionId}/${suit}.png`,
        })),
        promptUsed: `style:${session.style} | opt ${o}`,
      });
    }
    session.options = options;
    session.chosenId = undefined;
    return { sessionId: session.id, styleOptions: options };
  }

  async selectIconStyle(sessionId: string, styleOptionId: string): Promise<void> {
    const s = this.session(sessionId);
    if (!s.options.some((o) => o.id === styleOptionId)) {
      err("unknown_option", `unknown style option: ${styleOptionId}`);
    }
    s.chosenId = styleOptionId;
  }

  async generateStyleGuide(sessionId: string): Promise<CardStyleGuide> {
    const s = this.session(sessionId);
    if (!s.chosenId) err("no_style_selected", "no style selected");
    const guide: CardStyleGuide = {
      id: `sg-${s.id}`,
      version: 1,
      derivedFromStyleOptionId: s.chosenId!,
      promptUsed: `style guide derived from ${s.chosenId}`,
      borderChromeKey: `mock/${s.id}/style-guide/border-chrome.png`,
      cardBackKey: `mock/${s.id}/style-guide/card-back.png`,
      cardFrontLayout: {
        aspect: 0.66,
        suitIcon: { x: 0.05, y: 0.05, w: 0.18, h: 0.18 },
        cardImagery: { x: 0.1, y: 0.18, w: 0.8, h: 0.62 },
        title: { x: 0.0, y: 0.86, w: 1.0, h: 0.1 },
      },
      shaderAreas: [{ id: "uv-frame", rect: { x: 0, y: 0, w: 1, h: 1 }, kind: "foil" }],
      flourishes: [
        {
          assetKey: `mock/${s.id}/style-guide/flourish-1.png`,
          rect: { x: 0, y: 0, w: 0.15, h: 0.15 },
          rotation: 0,
        },
      ],
    };
    s.guide = guide;
    return guide;
  }

  async composeSampleCard(sessionId: string): Promise<SampleCard> {
    const s = this.session(sessionId);
    if (!s.guide) err("no_style_guide", "no style guide generated");
    const court = allCourtCards()[fnv1a(s.id + "court") % 16];
    const sample: SampleCard = {
      courtCard: court,
      cardImageryKey: `mock/${s.id}/sample/imagery.png`,
      frontKey: `mock/${s.id}/sample/front.png`,
      backKey: s.guide!.cardBackKey,
      approval: "pending",
    };
    s.sample = sample;
    return sample;
  }

  async getAsset(key: string): Promise<AssetResponse> {
    return { key, mime: "image/svg+xml", dataUrl: svgDataUrl(key) };
  }

  async approveDeck(sessionId: string): Promise<Deck> {
    const s = this.session(sessionId);
    if (!s.sample) err("nothing_to_approve", "nothing to approve");
    const chosen = s.options.find((o) => o.id === s.chosenId)!;
    const deck: Deck = {
      id: `deck-${s.id}`,
      active: true,
      suitIcons: chosen.suitIcons,
      styleGuide: s.guide!,
    };
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(ACTIVE_KEY, JSON.stringify(deck));
    }
    this.sessions.delete(sessionId);
    return deck;
  }

  async rejectAndRestart(sessionId: string): Promise<RestartInputs> {
    const s = this.session(sessionId);
    const inputs = { style: s.style };
    this.sessions.delete(sessionId);
    return inputs;
  }

  async getActiveDeck(): Promise<Deck | null> {
    if (typeof localStorage === "undefined") return null;
    const raw = localStorage.getItem(ACTIVE_KEY);
    return raw ? (JSON.parse(raw) as Deck) : null;
  }

  private session(id: string): MockSession {
    const s = this.sessions.get(id);
    if (!s) err("unknown_session", `unknown session: ${id}`);
    return s!;
  }
}

/** A deterministic colored SVG tile keyed by the asset key — stands in for real imagery. */
function svgDataUrl(key: string): string {
  const h = fnv1a(key);
  const hue = h % 360;
  const hue2 = (h >> 3) % 360;
  const svg = `<svg xmlns='http://www.w3.org/2000/svg' width='128' height='128'>
    <defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'>
      <stop offset='0' stop-color='hsl(${hue},60%,55%)'/>
      <stop offset='1' stop-color='hsl(${hue2},55%,35%)'/>
    </linearGradient></defs>
    <rect width='128' height='128' fill='url(#g)'/>
    <circle cx='64' cy='64' r='${28 + (h % 20)}' fill='hsla(${(hue + 180) % 360},70%,80%,0.7)'/>
  </svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}
