# Quickstart: Deck Creation

How to run and validate the feature end-to-end. By default everything runs on the
**FakeProvider + fixed seed**, so no network or credentials are required (constitution
Principles V, VII). Real generation is opt-in via configuration/env.

## Prerequisites

- Rust stable (1.80+) with `cargo`.
- Node.js (LTS) with `pnpm` (or `npm`) for the `frontend/`.
- Tauri 2 system prerequisites for your OS (webview deps).
- No AI credentials needed for the default (fake) path.

## Layout recap

- Core crates: `crates/{domain,providers,render,storage,deckforge}`
- Desktop shell: `src-tauri/`
- Frontend: `frontend/` (React + Vite + TS)

## Run the desktop app (fake provider)

```bash
# from repo root
cd frontend && pnpm install && cd ..
pnpm --dir frontend tauri dev      # or: cargo tauri dev
```

Expected: the Deck Creation wizard opens on the **Inputs** step.

## Manual end-to-end validation (maps to user stories)

1. **US1 — generate & choose icons**: enter any iconography rules + deck-style text, click
   Generate. → At least **3** distinct style options appear, each showing all four suits
   (SC-002). Click Regenerate → a fresh round appears, inputs preserved. Select one option.
2. **US2 — style guide**: proceed. → A card style guide is generated (border/chrome, card
   back, front layout, ≥1 shader/UV area, flourishes) without re-asking for style text.
3. **US3 — reveal & approve**: proceed. → A **court/face card** (Page/Knight/Queen/King)
   appears **face down**, then **flips** to reveal (SC-008). Approve → app reports an active
   deck. Reopen the app → the same deck loads (SC-003/004). Or Reject → returns to Inputs with
   inputs intact and no active deck (SC-005).

## Automated validation (test-first; all on fake provider + fixed seed)

```bash
# Rust: domain, render, storage, providers, and the end-to-end pipeline
cargo test

# Frontend unit (wizard state machine, client port, helpers)
pnpm --dir frontend test          # Vitest

# Component tests + living docs
pnpm --dir frontend storybook     # interactive
pnpm --dir frontend test-storybook  # CI run of stories

# UI flows against the MockDeckForgeClient (no Tauri/network)
pnpm --dir frontend playwright test
```

### What the automated suites prove

- **Determinism (SC-007)**: same seed → same chosen court card and same option ordering;
  rendered sample front/back are byte-stable offline (deck-bundle contract test).
- **Contract parity**: the Rust commands and the frontend `MockDeckForgeClient` satisfy the
  same `contracts/ipc-commands.md` shapes/error codes.
- **Approval gating (FR-018/SC-005)**: only approval sets the active deck; reject/abandon does
  not.
- **Headless pipeline (Principle VII)**: full icon→style-guide→sample→approve runs with no
  network and no credentials.

## Opt into real generation (optional)

```bash
export DECKFORGE_PROVIDER=<provider-id>     # selects the HTTP adapter
export DECKFORGE_API_KEY=...                # from env only; never logged/committed
pnpm --dir frontend tauri dev
```

Behavior is identical; only the provider changes (Principle I). Credentials stay in the
environment and out of logs and the repo (Principle VI).

## Success-criteria checklist (quick reference)

- [ ] ≥3 style options per round (SC-002)
- [ ] Visible progress within ~1s on each generation step; no silent hangs (SC-001/SC-006)
- [ ] Sample card starts face-down and flips before the approve prompt (SC-008)
- [ ] Approved deck persists and reloads on reopen (SC-003/SC-004)
- [ ] Rejected attempt → no active deck, inputs preserved (SC-005)
- [ ] Same seed reproduces the same deck look (SC-007)
