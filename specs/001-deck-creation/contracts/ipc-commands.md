# Contract: Frontend ↔ Core IPC Commands

The boundary between the TypeScript frontend and the Rust core. In the desktop app these are
Tauri `#[tauri::command]`s invoked via the `SibylClient` port; in Storybook/Playwright/web
the same port is backed by a mock/HTTP implementation. The command **names, inputs, and
outputs are the stable contract** — both implementations MUST honor them.

All commands are async. Errors are returned as a structured `{ code, message, retryable }`
(never thrown as opaque strings). Personal inputs are never echoed into logs.

## `generate_icon_styles`

Generate a round of suit-icon style options (FR-003/004/006).

- **Input**: `{ sessionId?: string, deckStyleText: string, count?: number /* default 3, min 3 */ }`
  (iconography rules are an app resource, not an input — see FR-001)
- **Output**: `{ sessionId: string, styleOptions: StyleOption[] }`  (`StyleOption[]` length ≥ 3)
- **Progress**: emits progress events keyed by `sessionId` while generating.
- **Errors**: `provider_failed` (retryable), `provider_unavailable` (retryable).
- **Notes**: If `sessionId` omitted, a new session + seed is created. Re-calling with the same
  `sessionId` regenerates options (preserves inputs; new seed-derived round). Image prompts are
  built via `compose_image_prompt(rules, style)` with the iconography rules authoritative over
  the deck-style text (FR-019); the composed prompt is recorded in provenance.

## `select_icon_style`

Record the owner's chosen style option (FR-005).

- **Input**: `{ sessionId: string, styleOptionId: string }`
- **Output**: `{ ok: true }`
- **Errors**: `unknown_session`, `unknown_option`.

## `generate_style_guide`

Auto-derive a prompt from the chosen style and generate the card style guide (FR-007/008).

- **Input**: `{ sessionId: string }`
- **Output**: `{ styleGuide: CardStyleGuide }`
- **Progress**: emits progress events.
- **Errors**: `no_style_selected`, `provider_failed` (retryable).
- **Notes**: The prompt is derived server-side from the chosen option — the frontend supplies
  no style text here (FR-008).

## `compose_sample_card`

Compose the canonical court-card sample (FR-009/010; always a court card).

- **Input**: `{ sessionId: string }`
- **Output**: `{ sampleCard: SampleCard }`  (includes `frontKey`, `backKey`, `courtCard`)
- **Progress**: emits progress events (JIT imagery generation + compositing).
- **Errors**: `no_style_guide`, `provider_failed` (retryable).
- **Notes**: Core selects the court card via the session's seeded RNG and records it. Returns
  storage keys; the frontend resolves them to displayable images via `get_asset`.

## `get_asset`

Resolve a storage key to displayable image bytes/URL (used for icons, front, back).

- **Input**: `{ key: string }`
- **Output**: `{ key: string, mime: string, data: bytes | url }`
- **Errors**: `unknown_key`.

## `approve_deck`

Approve the sample → persist the bundle and set it active (FR-012/018).

- **Input**: `{ sessionId: string }`
- **Output**: `{ deck: Deck /* active: true */ }`
- **Errors**: `nothing_to_approve`.
- **Notes**: Atomically writes the bundle and flips the active-deck pointer. Idempotent for a
  given approved session.

## `reject_and_restart`

Reject the sample and return to the input step with inputs preserved (FR-013).

- **Input**: `{ sessionId: string }`
- **Output**: `{ style: string }`  (the deck-style description, to repopulate the input)
- **Notes**: Leaves any existing active deck untouched (edge case); discards the draft's
  unapproved artifacts but returns the prior text inputs for editing.

## `get_active_deck`

Load the current active deck, if any (SC-003/004; used on app open and by future features).

- **Input**: `{}`
- **Output**: `{ deck: Deck | null }`

## Contract tests (Principle VII — written first)

- Each command has a Rust contract test using the **FakeProvider + fixed seed**: asserts shape,
  error codes, and determinism (same seed → same `courtCard`, same option ordering).
- The frontend `MockSibylClient` implements this same contract so Storybook/Playwright run
  without Tauri; a shared fixture set keeps mock and real implementations in agreement.
