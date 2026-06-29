# Implementation Plan: Deck Creation

**Branch**: `001-deck-creation` | **Date**: 2026-06-28 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-deck-creation/spec.md`

## Summary

Deck Creation is the **Preparation phase** of the personal tarot app (constitution
Principle III): a guided desktop wizard that establishes the owner's deck identity and writes
it as a reusable prepared-state bundle. The owner supplies two text inputs (iconography
*rules* + free-text *deck style*); the app generates several cohesive suit-icon **style
options** via a provider-agnostic AI seam; the owner picks one; the app auto-derives a prompt
to generate a **card style guide** (borders/chrome, card back, front layout, animated-shader
"UV" areas, flourishes); it composes a single **court/face card** sample (canonical CPU
render combining suit icon + style-guide front/back + just-in-time card imagery), presents it
face-down then flips it; on approval the bundle becomes the **active deck**, on rejection the
owner restarts. Built on the bound stack — Rust core + Tauri desktop shell + TypeScript/React
frontend — with all AI access behind a provider trait (fake provider for tests/dev), all
persistence behind a storage trait (filesystem default), seeded RNG + recorded provenance for
reproducibility, and a Tauri-decoupled frontend so the same UI runs in Storybook/Playwright
without the desktop shell.

## Technical Context

**Language/Version**: Rust stable (edition 2021, 1.80+) for the core and Tauri shell;
TypeScript 5.x for the frontend.

**Primary Dependencies**:
- Core: `tokio` (async), `serde`/`serde_json`, `thiserror`/`anyhow`, `rand` + `rand_chacha`
  (seeded ChaCha20 RNG), `tiny-skia` + `resvg`/`usvg` + `image` + `ab_glyph` (canonical CPU
  compositing & typography), `reqwest` (real provider adapter, async HTTP).
- Desktop: Tauri 2.x.
- Frontend: React 18 + Vite 5 + TypeScript; Vitest (unit), Storybook 8 (components),
  Playwright (UI flows).

**Storage**: Local filesystem by default, behind a `Store` trait (constitution III). Deck
bundles live under the OS app-data directory; tests use a temp dir. Swappable (e.g.,
`object_store`) without touching domain/render code.

**Testing**: `cargo test` (Rust unit + pipeline), Vitest (TS unit), Storybook stories
(components), Playwright (wizard flows). All run on the fake provider + fixed seed — no
network, no credentials (Principles V, VII).

**Target Platform**: Desktop (Linux/macOS/Windows) via Tauri. Frontend is also runnable as a
plain web app against a mock/HTTP backend (Principle II + frontend-portability constraint).

**Project Type**: Desktop application — Cargo workspace of library crates + a thin Tauri
shell + a separate TypeScript frontend.

**Performance Goals**: UI stays responsive during generation (all AI/render work off the UI
thread); visible progress feedback within ~1s of starting any step (SC-001, SC-006); card
flip animates at ~60fps; canonical sample-card render completes in well under a second once
imagery is available (CPU compositing). End-to-end latency is dominated by the external AI
provider and is surfaced as progress, not blocked-on.

**Constraints**: Provider-agnostic (no vendor types outside the adapter); only an approved
result becomes active; personal inputs + deck stay local and out of logs; generation is
reproducible from recorded provenance (the bundle stores the actual generated image bytes so
re-display is identical even though providers are non-deterministic).

**Scale/Scope**: Single owner, single active deck. Standard four suits (16 court cards). ~3
wizard steps, ~3 style options/round. Not a high-throughput service.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Constitution v1.3.1. Initial evaluation:

| Principle | Gate | Status |
|-----------|------|--------|
| Guiding Experience (Surprise / Connections / Visual Inspiration) | Feature must amplify the triad | **PASS** — the flip-reveal and varied style options serve Surprise; the whole feature *is* visual-identity creation; the personal deck-style text seeds future connections. |
| I. Provider-Agnostic AI | All AI behind a trait; fake provider exists | **PASS** — `TextProvider`/`ImageProvider` traits; `FakeProvider` is the test/dev default; one HTTP adapter behind the trait. |
| II. Local-First, Cloud-Parity | Runs locally; core behind interfaces | **PASS** — this is the local Preparation phase; core logic lives in interface-bounded crates reusable elsewhere. |
| III. Prepared State, Stateless Draws | Produces a stored, referenced bundle; storage seam; filesystem default | **PASS** — deck creation *is* Preparation; output is the prepared-state bundle via `Store` (filesystem default, swappable). |
| IV. Coherent Visual Identity | Identity is explicit, versioned, composited deterministically | **PASS** — style guide + icon set persisted and versioned; canonical card render composites AI imagery with style-guide assets in the `render` crate. |
| V. Reproducible Generation | Seedable RNG; recorded inputs; replayable | **PASS** — ChaCha20 seed drives court-card pick & prompt variation; provenance + generated bytes stored in the bundle. |
| VI. Personal Data Stays Private | Inputs/deck owner-controlled, out of repo & logs; record what's sent | **PASS** — stored locally under app-data; redacting debug on personal fields; provenance records exactly what was sent to the provider; provider keys via env, never logged/committed. |
| VII. Test-First Discipline | Tests first; unit/Storybook/Playwright on headless substrate | **PASS** — TDD across crates and frontend; fake provider + fixed seed make the whole pipeline headless. |
| Tech stack (Rust/Tauri/TS-over-JS/portable frontend) | Bound stack honored | **PASS** — exactly this stack; frontend decoupled from the Tauri runtime via a platform port. |

**Result**: No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/001-deck-creation/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── ipc-commands.md      # Frontend ↔ core command contract (the IPC seam)
│   ├── deck-bundle.md       # Prepared-state bundle format (consumed by future features)
│   └── provider-interface.md# Provider-agnostic AI trait contract
├── checklists/
│   └── requirements.md  # Spec quality checklist (from /speckit-specify)
└── tasks.md             # /speckit-tasks output (not created here)
```

### Source Code (repository root)

```text
crates/
├── domain/            # Pure: deck/suit/court-card model, style-option & selection logic,
│                      #   seeded draw/RNG, provenance types. No IO. (cargo test)
├── providers/         # TextProvider/ImageProvider traits, FakeProvider (deterministic),
│                      #   and one HTTP adapter behind the trait. (cargo test w/ fake)
├── render/            # Canonical CPU compositing (tiny-skia/resvg/image/ab_glyph):
│                      #   suit icons, style-guide layout, sample court-card front/back.
├── storage/           # Store trait + filesystem impl (default). Bundle read/write.
└── sibyl/         # Orchestration use-cases: generate_icon_styles, generate_style_guide,
                       #   compose_sample_card, approve_deck. Wires domain+providers+render+storage.

src-tauri/             # Thin Tauri 2 shell: #[tauri::command] wrappers over sibyl,
├── src/               #   app-data path resolution, provider/store construction from config.
└── tauri.conf.json

frontend/              # TypeScript + React + Vite web app (runs in Tauri OR plain web)
├── src/
│   ├── components/    # Presentational components (Storybook stories = component tests)
│   ├── views/         # Wizard steps: Inputs, StyleChoice, StyleGuide, Reveal/Approve
│   ├── platform/      # SibylClient port: tauri-invoke impl + mock/http impl
│   └── state/         # Wizard state machine (TS), provider-agnostic of transport
├── tests/             # Vitest unit; Playwright e2e (against mock backend)
└── .storybook/

tests/
└── integration/       # Rust end-to-end pipeline tests (fake provider, fixed seed)
```

**Structure Decision**: A Cargo workspace isolates the constitution's seams as crates
(`domain`, `providers`, `render`, `storage`) with orchestration in `sibyl`; `src-tauri`
is a thin shell depending on those crates so the core stays portable to a future FaaS draw
service. The `frontend` is a standalone React/Vite app that talks to the core only through a
`SibylClient` port (Tauri-invoke implementation for desktop, mock/HTTP implementation for
Storybook/Playwright/web) — satisfying the frontend-portability constraint and making the UI
testable without the desktop shell. May start consolidated and split further if a crate grows;
the seams (traits) are the non-negotiable part, not the crate count.

## Complexity Tracking

> No constitution violations. No entries required.
