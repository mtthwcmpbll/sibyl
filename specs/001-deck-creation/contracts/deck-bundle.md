# Contract: Deck Bundle (Prepared-State Format)

The durable output of Deck Creation and the prepared-state bundle future features consume
(constitution Principle III). It is accessed only through the `Store` trait; the default
backend is the local filesystem, but the layout is backend-neutral (keys, not paths). The
manifest is JSON so it travels to a future web/FaaS draw service unchanged.

## Layout (filesystem default)

```text
<app-data>/decks/
├── active.json                 # { "activeDeckId": "<uuid>" | null }
└── <deckId>/
    ├── manifest.json           # the Deck record (see schema below)
    ├── icons/
    │   ├── cups.png
    │   ├── wands.png
    │   ├── swords.png
    │   └── pentacles.png
    ├── style-guide/
    │   ├── border-chrome.png
    │   ├── card-back.png
    │   ├── flourish-*.png
    │   └── style-guide.json    # layout, shaderAreas, flourish placements
    └── sample/
        ├── front.png           # canonical composited court-card front
        └── back.png
```

Drafts live under `<app-data>/decks/_drafts/<sessionId>/` with the same shape; only approval
copies/promotes a draft to `<deckId>/` and updates `active.json`.

## `manifest.json` schema (the Deck record)

```jsonc
{
  "id": "uuid",
  "active": true,
  "createdAt": "2026-06-28T00:00:00Z",
  "suitSet": ["cups", "wands", "swords", "pentacles"],
  "suitIcons": [
    { "suit": "cups", "imageKey": "decks/<id>/icons/cups.png" }
    // … one per suit
  ],
  "styleGuide": {
    "id": "uuid",
    "version": 1,
    "derivedFromStyleOptionId": "opt-…",
    "borderChromeKey": "decks/<id>/style-guide/border-chrome.png",
    "cardBackKey": "decks/<id>/style-guide/card-back.png",
    "cardFrontLayout": {
      "aspect": 0.6,
      "slots": {
        "suitIcon":   { "x": 0.0, "y": 0.0, "w": 0.2, "h": 0.2 },
        "cardImagery":{ "x": 0.1, "y": 0.15, "w": 0.8, "h": 0.6 },
        "title":      { "x": 0.0, "y": 0.85, "w": 1.0, "h": 0.1 }
      }
    },
    "shaderAreas": [
      { "id": "uv-frame", "rect": { "x": 0, "y": 0, "w": 1, "h": 1 }, "kind": "foil" }
    ],
    "flourishes": [
      { "assetKey": "decks/<id>/style-guide/flourish-1.png",
        "rect": { "x": 0.0, "y": 0.0, "w": 0.15, "h": 0.15 }, "rotation": 0 }
    ]
  },
  "provenance": {
    "seed": 1234567890,
    "chosenStyleOptionId": "opt-…",
    "courtCardShown": "queen-of-cups",
    "providerId": "<provider-id>",
    "modelId": "<model-id>",
    "assetVersions": { "styleGuide": 1 },
    "prompts": {
      "icons": "…",
      "styleGuide": "…",
      "sampleImagery": "…"
    }
    // iconographyRules and deckStyleText are stored here too, but are PERSONAL:
    // present in the local bundle, never logged, never committed to the repo.
  }
}
```

## Invariants

- **Single active deck**: `active.json.activeDeckId` references at most one deck; set only by
  `approve_deck` (FR-018).
- **Self-contained**: every key referenced in `manifest.json` resolves within the bundle, so
  the bundle is "bakeable" into a deployment (Principle III) and reproduces the same look
  offline (Principle V).
- **Court card**: `provenance.courtCardShown` is always one of the 16 court cards
  (clarification 2026-06-28).
- **Privacy**: a bundle containing personal provenance is sensitive — excluded from version
  control and protected wherever stored/baked (Principle VI).
- **Versioned identity**: `styleGuide.version` increments on any future identity change so
  prior cards remain attributable (Principle IV).

## Contract tests (written first)

- Round-trip: write a deck via `Store`, read it back, assert manifest + all referenced keys
  resolve.
- Active pointer: approving sets exactly one active deck; a second approval reassigns it; a
  rejected session leaves it unchanged.
- Offline reproduction: with the bundle present and the network disabled, the sample front/back
  render identically (byte-stable) — proves cached-bytes reproducibility.
