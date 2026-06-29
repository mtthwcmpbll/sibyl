# Deckforge — a personal tarot deck

An app that uses an LLM (and image generation) to design and render a tarot deck made for
one person. This repo implements the first feature, **Deck Creation** (the Preparation phase):
generate suit-icon styles, choose one, auto-generate a card style guide, then compose a
court-card sample, flip it, and approve it as your deck.

See the constitution (`.specify/memory/constitution.md`) and the feature design under
`specs/001-deck-creation/`.

## Architecture

A Cargo workspace of pure-Rust crates behind the constitution's seams, a Tauri desktop shell,
and a portable React/TypeScript frontend.

```
crates/
  domain/     suits, court cards, seeded RNG, data model (personal fields redacted)
  providers/  TextProvider/ImageProvider traits + deterministic FakeProvider
  storage/    Store trait + filesystem backend + deck-bundle/active-pointer
  render/     deterministic CPU compositing (the canonical card image)
  deckforge/  orchestration: compose_image_prompt, generate_icon_styles, style guide,
              sample, approve/reject/get_active_deck
  app-core/   Tauri-free command layer: arg + error-code mapping, asset encoding,
              provider/storage construction — tested headlessly
src-tauri/    thinnest Tauri 2 shell: 8 one-line command wrappers over app-core + the
              Builder wiring (this crate alone needs system webkit to compile)
frontend/     React + Vite + TS; DeckForgeClient port (Tauri + mock impls), wizard,
              components, views, Storybook, Playwright
```

Key properties (from the constitution):

- **Provider-agnostic** — all AI behind traits; a `FakeProvider` runs everything offline.
- **Prepared state, stateless draws** — deck creation writes a self-contained bundle via a
  swappable `Store` (filesystem by default).
- **Reproducible** — seeded ChaCha20 RNG + recorded provenance; the canonical render is
  byte-stable.
- **Private** — personal inputs are redacted in logs and kept local.
- **Test-first** — `cargo test`, Vitest, Storybook, Playwright, all on the fake provider.

## Run the tests (no credentials needed)

```bash
# Rust core + command layer (44 tests)
cargo test

# Frontend unit + UI-flow tests (Vitest)
pnpm --dir frontend install
pnpm --dir frontend test

# Component stories (Storybook)
pnpm --dir frontend build-storybook

# End-to-end UI flows (Playwright, real browser, mock backend)
pnpm --dir frontend exec playwright install chromium
pnpm --dir frontend exec playwright test
```

## Run the desktop app

The desktop shell builds separately because Tauri needs system libraries on Linux:

```bash
sudo apt install libwebkit2gtk-4.1-dev libsoup-3.0-dev build-essential \
    curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev
cargo install tauri-cli --version '^2'
cd src-tauri && cargo tauri dev
```

By default it uses the offline `FakeProvider`. Real generation is opt-in via environment
(credentials come from the environment only and are never logged):

```bash
export DECKFORGE_PROVIDER=<provider-id>
export DECKFORGE_API_KEY=...
```

## Performance notes

Generation is the only slow step and is always off the UI thread; the wizard surfaces a busy
state and a retryable error for every step (no silent hangs). The reveal flip is a CSS 3D
transform over the canonical front/back images, and the canonical render is pure-CPU and
byte-stable, so the same deck looks identical on every open and on any platform.
