# Phase 1 Data Model: Deck Creation

Entities derived from the spec's Key Entities and Functional Requirements. Types are described
domain-neutrally (the Rust structs and the TS mirrors follow the same shape). Image assets are
referenced by storage key, not embedded.

## Entity: DeckCreationSession (draft)

The in-progress wizard state. Persisted under a draft key so retries/back-navigation don't
regenerate; never "active".

| Field | Type | Notes / Validation |
|-------|------|--------------------|
| `id` | string (uuid) | Draft session id. |
| `seed` | u64 | ChaCha20 seed for this session (reproducibility, Principle V). |
| `iconographyRules` | string | FR-001; optional (defaults apply when blank). Personal — redacted in logs. |
| `deckStyleText` | string | FR-002; optional. Personal — redacted in logs. |
| `styleOptions` | StyleOption[] | Current round's options (FR-003); ≥3. |
| `chosenStyleOptionId` | string? | Set when the owner selects (FR-005). |
| `styleGuide` | CardStyleGuide? | Generated after selection (FR-007/008). |
| `sampleCard` | SampleCard? | Composed court-card preview (FR-009). |
| `status` | enum | `inputs` \| `choosingStyle` \| `styleGuide` \| `review` \| `approved` \| `discarded`. |
| `createdAt` / `updatedAt` | timestamp | Provenance/audit. |

**State transitions**: `inputs → choosingStyle` (generate options) → `styleGuide` (select
option) → `review` (compose sample) → `approved` (approve, FR-012) or back to `inputs`
(reject/restart, FR-013). Regenerating options loops within `choosingStyle` (FR-006). Only
`approved` produces an active Deck (FR-018).

## Entity: StyleOption (Suit-Icon Set)

One cohesive candidate covering every suit; the owner picks exactly one.

| Field | Type | Notes / Validation |
|-------|------|--------------------|
| `id` | string | Option id, unique within the round. |
| `label` | string | Short human label (e.g., "Option A"). |
| `suitIcons` | SuitIcon[] | One per suit; all suits present (FR-004). |
| `promptUsed` | string | The generation prompt (provenance). |

### Value: SuitIcon

| Field | Type | Notes |
|-------|------|-------|
| `suit` | enum | Standard: `cups` \| `wands` \| `swords` \| `pentacles` (rules may override set). |
| `imageKey` | string | Storage key of the generated icon image. |

## Entity: CardStyleGuide

The card-system definition derived from the chosen style (FR-007/008). Versioned (Principle IV).

| Field | Type | Notes / Validation |
|-------|------|--------------------|
| `id` | string | Style-guide id. |
| `version` | int | Visual-identity version (Principle IV — versioned identity). |
| `derivedFromStyleOptionId` | string | The chosen option it was generated from. |
| `promptUsed` | string | Auto-derived prompt (FR-008; provenance). |
| `borderChromeKey` | string | Border/chrome asset (FR-008). |
| `cardBackKey` | string | Card-back asset. |
| `cardFrontLayout` | FrontLayout | Front layout definition (icon slot, imagery slot, title area). |
| `shaderAreas` | ShaderArea[] | Designated animated-shader/"UV coating" regions (FR-008). |
| `flourishes` | FlourishRef[] | Decorative flourish assets/placements. |

### Value: FrontLayout / ShaderArea / FlourishRef

- `FrontLayout`: slot rectangles (normalized 0–1 coords) for `suitIcon`, `cardImagery`,
  `title`, plus canvas aspect ratio.
- `ShaderArea`: `{ id, rect (normalized), kind }` — where the presentation layer may later
  apply animated shaders; this feature only *designates* them.
- `FlourishRef`: `{ assetKey, rect, rotation }`.

## Entity: SampleCard

The single composed court/face card preview (FR-009/010; clarified: always a court card).

| Field | Type | Notes / Validation |
|-------|------|--------------------|
| `courtCard` | CourtCardId | One of 16: `{suit} × {page,knight,queen,king}`. Chosen by seeded RNG (recorded). |
| `cardImageryKey` | string | Just-in-time generated figure imagery for this court card. |
| `frontKey` | string | Canonical composited front (icon + style-guide front + imagery + chrome). |
| `backKey` | string | Card back from the style guide. |
| `approval` | enum | `pending` \| `approved` \| `rejected` (FR-011). |

## Entity: Deck (Deck Identity) — the prepared-state bundle

The durable, approved outcome; the prepared-state bundle future features consume (Principle
III). See `contracts/deck-bundle.md` for the on-disk format.

| Field | Type | Notes / Validation |
|-------|------|--------------------|
| `id` | string | Deck id. |
| `active` | bool | Exactly one active deck at a time (Assumptions); set only on approval. |
| `suitIcons` | SuitIcon[] | The chosen style's suit icons. |
| `styleGuide` | CardStyleGuide | The approved style guide. |
| `provenance` | GenerationProvenance | Inputs + identifiers sufficient to explain/regenerate. |
| `createdAt` | timestamp | Approval time. |

## Entity: GenerationProvenance

Recorded for the deck and for each generated artifact (FR-014; Principle V/VI).

| Field | Type | Notes |
|-------|------|-------|
| `seed` | u64 | Session seed. |
| `iconographyRules` | string | What was provided (personal; protected, not logged). |
| `deckStyleText` | string | What was provided (personal; protected, not logged). |
| `chosenStyleOptionId` | string | Selected option. |
| `courtCardShown` | CourtCardId | Sample card the deck was approved against. |
| `providerId` / `modelId` | string | Identifier of the provider/model used (no secrets). |
| `assetVersions` | map | Style-guide/asset versions for traceability. |
| `prompts` | map | The **composed** prompts actually sent (records exactly what left the machine; rules authoritative over style per FR-019). |

## Relationships

- `DeckCreationSession` 1—* `StyleOption`; selects 1 → derives 1 `CardStyleGuide` → composes 1
  `SampleCard`.
- On approval, a `DeckCreationSession` produces 1 `Deck` embedding the chosen `SuitIcon[]`, the
  `CardStyleGuide`, and `GenerationProvenance`.
- `Deck.active = true` for at most one deck (single active deck).

## Validation & invariants

- A `StyleOption` is valid only if it has a `SuitIcon` for every suit in the active suit set.
- A `CardStyleGuide` must include border/chrome, card back, a front layout, ≥1 shader area, and
  flourishes (FR-008) before a `SampleCard` can be composed.
- `SampleCard.courtCard` MUST be a court card (Page/Knight/Queen/King) — never a pip/minor or
  Major Arcana (clarification 2026-06-28).
- A `Deck` becomes `active` only via approval; rejected/abandoned sessions never set `active`
  (FR-018).
- Personal fields (`iconographyRules`, `deckStyleText`) MUST use redacting debug/log
  representations (Principle VI).
