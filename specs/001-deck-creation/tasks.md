---

description: "Task list for Deck Creation"
---

# Tasks: Deck Creation

**Input**: Design documents from `/specs/001-deck-creation/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: REQUIRED by the project constitution (Principle VII, Test-First Discipline). Every
story includes test tasks written FIRST and observed to FAIL before implementation. Non-UI
behavior → unit tests (`cargo test` / Vitest); components → Storybook stories; UI flows →
Playwright. All run on the FakeProvider + fixed seed (no network, no credentials).

**Organization**: Tasks are grouped by user story. Stories are independently testable using
fixtures, even though the wizard runs them in sequence.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on incomplete tasks)
- **[Story]**: US1 / US2 / US3 (user-story phases only)
- Every task includes an exact file path

## Path Conventions

Cargo workspace + Tauri shell + React/Vite frontend (see plan.md → Project Structure):
`crates/{domain,providers,render,storage,deckforge}`, `src-tauri/`, `frontend/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize the workspace, shell, frontend, and the test toolchain.

- [X] T001 Create Cargo workspace with member crate skeletons (domain, providers, render, storage, deckforge) in `Cargo.toml` and `crates/*/Cargo.toml` + `crates/*/src/lib.rs`
- [ ] T002 [P] Initialize Tauri 2 desktop shell in `src-tauri/` (`src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/src/main.rs`) depending on the workspace crates
- [ ] T003 [P] Initialize React 18 + Vite 5 + TypeScript (strict) app in `frontend/` (`frontend/package.json`, `frontend/tsconfig.json`, `frontend/vite.config.ts`, `frontend/src/App.tsx`)
- [X] T004 [P] Configure Rust formatting/linting in `rustfmt.toml` and a clippy lint script in `scripts/lint.sh`
- [ ] T005 [P] Configure ESLint + Prettier + TS strict in `frontend/.eslintrc.cjs` and `frontend/.prettierrc`
- [ ] T006 [P] Set up Vitest in `frontend/vitest.config.ts` with a sample passing test in `frontend/tests/smoke.test.ts`
- [ ] T007 [P] Set up Storybook 8 in `frontend/.storybook/main.ts` and `frontend/.storybook/preview.ts`
- [ ] T008 [P] Set up Playwright in `frontend/playwright.config.ts` configured to run against the mock backend
- [ ] T009 [P] Add shared `FIXED_SEED` constant + test fixtures in `crates/domain/src/lib.rs` and `frontend/tests/fixtures.ts`

**Checkpoint**: Workspace builds, frontend runs, all four test runners execute an empty/sample suite.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: The constitution's seams (provider, storage, RNG, domain model, render
primitives, IPC plumbing, frontend client port) that every story depends on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T010 [P] Unit tests for `Suit` and the 16 `CourtCardId`s (write first, must fail) in `crates/domain/tests/court.rs`
- [X] T011 Implement `Suit` + `CourtCardId` + active suit set in `crates/domain/src/suit.rs` and `crates/domain/src/court.rs`
- [X] T012 [P] Unit test: seeded ChaCha20 RNG determinism + court-card selection (write first) in `crates/domain/tests/rng.rs`
- [X] T013 Implement seeded RNG helper + `select_court_card(seed)` in `crates/domain/src/rng.rs`
- [X] T014 [P] Unit tests: model (de)serialization + invariants + redacted Debug for personal fields (write first) in `crates/domain/tests/model.rs`
- [X] T015 Implement domain types (Session, StyleOption, CardStyleGuide, SampleCard, Deck, GenerationProvenance) with redacting `Debug` on personal fields in `crates/domain/src/{session.rs,style.rs,deck.rs,provenance.rs}`
- [X] T016 [P] Provider contract tests against FakeProvider (determinism by (prompt,seed), error mapping, stable `id()`) (write first) in `crates/providers/tests/fake.rs`
- [X] T017 Implement `ProviderError`/`ProviderId` + `TextProvider`/`ImageProvider` traits in `crates/providers/src/error.rs` and `crates/providers/src/traits.rs`
- [X] T018 Implement `FakeProvider` (deterministic placeholder text/image from `(prompt,seed)`) in `crates/providers/src/fake.rs`
- [X] T019 [P] Store round-trip tests (filesystem impl, temp dir) (write first) in `crates/storage/tests/fs.rs`
- [X] T020 Implement `Store` trait + filesystem implementation in `crates/storage/src/store.rs` and `crates/storage/src/fs.rs`
- [X] T021 [P] Bundle read/write + single-active-pointer tests per `contracts/deck-bundle.md` (write first) in `crates/storage/tests/bundle.rs`
- [X] T022 Implement bundle (de)serialization + `active.json` handling in `crates/storage/src/bundle.rs`
- [X] T023 [P] Unit test: deterministic layer compositing (write first) in `crates/render/tests/compositor.rs`
- [X] T024 Implement compositing primitives (Pixmap layering, decode/encode, normalized-slot placement) in `crates/render/src/compositor.rs` and `crates/render/src/layout.rs`
- [X] T025 Implement `deckforge` orchestration skeleton + error type + dependency injection of providers/store/RNG in `crates/deckforge/src/lib.rs` and `crates/deckforge/src/error.rs`
- [ ] T026 Implement Tauri command plumbing: registry, provider+store construction from config/env, app-data path resolution in `src-tauri/src/main.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/config.rs`
- [ ] T027 [P] Contract test for `get_asset` (resolves a stored key; `unknown_key` error) (write first) in `src-tauri/tests/get_asset.rs`
- [ ] T028 Implement `get_asset` command in `src-tauri/src/commands.rs`
- [ ] T029 [P] Vitest spec: wizard state-machine transitions (write first) in `frontend/tests/wizard.test.ts`
- [ ] T030 Implement `DeckForgeClient` port interface + shared TS types mirroring `contracts/ipc-commands.md` in `frontend/src/platform/client.ts`
- [ ] T031 Implement `MockDeckForgeClient` (fixture-backed, honors the IPC contract) in `frontend/src/platform/mockClient.ts`
- [ ] T032 Implement `TauriDeckForgeClient` (via `@tauri-apps/api` invoke) in `frontend/src/platform/tauriClient.ts`
- [ ] T033 Implement wizard state machine + app shell/step router in `frontend/src/state/wizard.ts` and `frontend/src/App.tsx`

**Checkpoint**: Seams exist and are unit-tested on the fake provider; the frontend can drive a no-op wizard against the mock client. User stories can now begin.

---

## Phase 3: User Story 1 - Generate and choose a suit-icon style (Priority: P1) 🎯 MVP

**Goal**: From the two text inputs, generate ≥3 cohesive suit-icon style options and let the
owner select one.

**Independent Test**: With fixed inputs + seed, `generate_icon_styles` returns ≥3 options each
covering all suits; the UI shows them and records exactly one selection; regenerate preserves
inputs.

### Tests for User Story 1 (REQUIRED - Principle VII) ⚠️

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T034 [P] [US1] Rust contract test: `generate_icon_styles` returns ≥3 options, all suits present, deterministic by seed; `select_icon_style` records choice — in `src-tauri/tests/us1_icons.rs`
- [X] T035 [P] [US1] Rust unit test: `deckforge` generate-icons use-case (per-suit prompt build, fake provider, ≥3 options) in `crates/deckforge/tests/icons.rs`
- [ ] T036 [P] [US1] Storybook stories: `StyleOption` and `StyleChoice` grid (loading / options / regenerating states) in `frontend/src/components/StyleOption.stories.tsx`
- [ ] T037 [P] [US1] Playwright: inputs → generate → ≥3 options → regenerate (inputs preserved) → select → proceed (mock backend) in `frontend/tests/us1-style-choice.spec.ts`
- [X] T069 [P] [US1] Rust unit test: `compose_image_prompt` makes iconography rules authoritative over deck-style text (write first) in `crates/deckforge/tests/prompt.rs` (FR-019)
- [ ] T071 [P] [US1] Failure-path tests: a provider error surfaces a retryable error and preserves entered inputs/selection — Rust contract in `src-tauri/tests/failure_retry.rs` and Playwright in `frontend/tests/failure-retry.spec.ts` (FR-017, SC-006; pattern reused by US2/US3)

### Implementation for User Story 1

- [X] T070 [US1] Implement `compose_image_prompt(rules, style)` via `TextProvider` (rules authoritative; output reused by icon, style-guide, and sample image generation) in `crates/deckforge/src/prompt.rs` (FR-019)
- [X] T038 [US1] Implement `generate_icon_styles` use-case (per-suit prompts via `compose_image_prompt` (T070), provider calls, store icons, ≥3 seeded options) in `crates/deckforge/src/icons.rs`
- [X] T039 [US1] Implement `select_icon_style` use-case (record chosen option on session) in `crates/deckforge/src/icons.rs`
- [ ] T040 [US1] Wire `generate_icon_styles` + `select_icon_style` Tauri commands (with progress events + retryable errors) in `src-tauri/src/commands.rs`
- [ ] T041 [P] [US1] `InputsView` (iconography rules + deck-style text, optional) in `frontend/src/views/InputsView.tsx`
- [ ] T042 [P] [US1] `StyleChoiceView` (option grid, select, regenerate, progress, retry) in `frontend/src/views/StyleChoiceView.tsx`
- [ ] T043 [US1] Wire US1 views to `DeckForgeClient` + wizard transitions (progress + retry on `provider_failed`) in `frontend/src/state/wizard.ts`

**Checkpoint**: US1 is independently demoable — enter inputs, get options, pick one.

---

## Phase 4: User Story 2 - Generate the card style guide (Priority: P2)

**Goal**: Auto-derive a prompt from the chosen style and generate the card style guide
(border/chrome, card back, front layout, ≥1 shader/UV area, flourishes).

**Independent Test**: Given a fixed chosen style, `generate_style_guide` derives its own
prompt and returns a guide containing all required elements; `no_style_selected` errors when
none chosen.

### Tests for User Story 2 (REQUIRED - Principle VII) ⚠️

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T044 [P] [US2] Rust contract test: `generate_style_guide` derives prompt from chosen style, returns border/chrome + back + front layout + ≥1 shader area + flourishes; `no_style_selected` error — in `src-tauri/tests/us2_style_guide.rs`
- [X] T045 [P] [US2] Rust unit test: `deckforge` style-guide use-case (prompt auto-derived, fake provider, versioned guide) in `crates/deckforge/tests/style_guide.rs`
- [ ] T046 [P] [US2] Storybook stories: `StyleGuidePreview` (loading / complete) in `frontend/src/components/StyleGuidePreview.stories.tsx`
- [ ] T047 [P] [US2] Playwright: selected style → generate guide → preview shows required elements (mock backend) in `frontend/tests/us2-style-guide.spec.ts`

### Implementation for User Story 2

- [X] T048 [US2] Implement style-guide prompt derivation from the chosen option in `crates/deckforge/src/style_guide.rs`
- [X] T049 [US2] Implement `generate_style_guide` use-case: generate/store assets, assemble front layout + shader areas + flourishes, version the guide — in `crates/deckforge/src/style_guide.rs` and `crates/render/src/layout.rs`
- [ ] T050 [US2] Wire `generate_style_guide` Tauri command (progress + retryable errors) in `src-tauri/src/commands.rs`
- [ ] T051 [US2] `StyleGuideView` (preview + proceed) wired to client in `frontend/src/views/StyleGuideView.tsx`

**Checkpoint**: US1 + US2 work — choose a style and get a coherent card style guide.

---

## Phase 5: User Story 3 - Preview, flip, and approve the sample card (Priority: P3)

**Goal**: Compose a court/face-card sample (suit icon + style-guide front/back + JIT imagery),
show it face-down, flip to reveal, then approve (→ active deck) or reject (→ restart).

**Independent Test**: Given fixed chosen icons + style guide, `compose_sample_card` returns a
court card (seeded) with front/back keys; the UI shows face-down → flip; approve persists a
single active deck; reject leaves no active deck and preserves inputs.

### Tests for User Story 3 (REQUIRED - Principle VII) ⚠️

> **Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T052 [P] [US3] Rust contract tests: `compose_sample_card` (always a court card, seeded, returns front/back keys), `approve_deck` (sets exactly one active), `reject_and_restart` (preserves inputs, no active), `get_active_deck` — in `src-tauri/tests/us3_commands.rs`
- [X] T053 [P] [US3] Rust unit test: compose use-case builds court-card front (icon + front + imagery + chrome) and back; byte-stable offline (reproducibility) in `crates/deckforge/tests/sample.rs`
- [X] T054 [P] [US3] Rust unit test: approve promotes draft → bundle + flips active pointer; reject leaves existing active unchanged in `crates/deckforge/tests/approve.rs`
- [ ] T055 [P] [US3] Storybook stories: `CardFlip` (face-down → revealed) in `frontend/src/components/CardFlip.stories.tsx`
- [ ] T056 [P] [US3] Playwright: compose → face-down → flip → approve → active deck persists on reload; reject → back to inputs with no active deck (mock backend) in `frontend/tests/us3-reveal-approve.spec.ts`

### Implementation for User Story 3

- [X] T057 [US3] Implement `compose_sample_card` use-case: seeded court-card pick, JIT imagery via provider, canonical compositing of front + back, store sample — in `crates/deckforge/src/sample.rs` and `crates/render/src/card.rs`
- [X] T058 [US3] Implement `approve_deck` (promote draft → bundle, set active), `reject_and_restart` (discard draft artifacts, keep inputs), `get_active_deck` — in `crates/deckforge/src/approve.rs`
- [X] T072 [US3] Assemble `GenerationProvenance` (rules, style, seed, chosen option, court card, provider/model id, composed prompts sent) into the draft and persist it in the approved bundle; unit test asserts it is sufficient to regenerate and that composed prompts are recorded — in `crates/deckforge/src/approve.rs` and `crates/deckforge/tests/provenance.rs` (FR-014/FR-019; Principles V/VI)
- [ ] T059 [US3] Wire `compose_sample_card` / `approve_deck` / `reject_and_restart` / `get_active_deck` Tauri commands in `src-tauri/src/commands.rs`
- [ ] T060 [P] [US3] `CardFlip` component (face-down → flip reveal animation, ~60fps) in `frontend/src/components/CardFlip.tsx`
- [ ] T061 [US3] `RevealView` (face-down card, flip, approve/reject) wired to client in `frontend/src/views/RevealView.tsx`
- [ ] T062 [US3] Load active deck via `get_active_deck` on app startup in `frontend/src/App.tsx`

**Checkpoint**: Full feature works end-to-end — create, reveal, approve/reject, persist.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: End-to-end verification and constitution-alignment hardening.

- [ ] T063 [P] End-to-end Rust pipeline integration test (fake provider, fixed seed): inputs → icons → style guide → sample → approve produces an active bundle in `tests/integration/pipeline.rs`
- [ ] T064 [P] Provider-seam guard test: assert no vendor types referenced outside the `providers` adapter module in `crates/providers/tests/seam.rs`
- [ ] T065 [P] Privacy check: assert personal fields are redacted in logs/Debug output (Principle VI) in `crates/domain/tests/redaction.rs`
- [ ] T066 [P] Error/empty/loading-state polish across all views in `frontend/src/views/`
- [ ] T067 [P] Run and validate `specs/001-deck-creation/quickstart.md` end-to-end (fake provider)
- [ ] T068 UX/perf pass: progress visible within ~1s per step, flip ~60fps, UI never blocks — adjustments in `frontend/src/state/wizard.ts` and affected views

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: no dependencies.
- **Foundational (Phase 2)**: depends on Setup; BLOCKS all user stories.
- **User Stories (Phases 3–5)**: each depends only on Foundational. The wizard chains them at
  runtime (US2 consumes US1's chosen style; US3 consumes US2's guide), but each is
  independently testable via fixtures, so they may be built in parallel by different people
  once Foundational is done. Recommended build order = priority order P1 → P2 → P3.
- **Polish (Phase 6)**: depends on the user stories being implemented.

### Within Each User Story

- Tests (Storybook/Playwright/unit/contract) MUST be written and FAIL before implementation (Principle VII).
- Rust use-case → Tauri command wiring → frontend view wiring.
- Components (Storybook) before the views that compose them.

### Story Dependencies

- **US1 (P1)**: foundational only — the MVP slice.
- **US2 (P2)**: foundational only; runtime-consumes US1 output (fixtures make it independently testable).
- **US3 (P3)**: foundational only; runtime-consumes US2 output (fixtures make it independently testable).

## Parallel Opportunities

- All Setup tasks marked [P] (T002–T009) can run together after T001.
- In Foundational, the test-first tasks across different crates are parallel: T010, T012, T014, T016, T019, T021, T023, T027, T029. Each paired implementation runs after its own test.
- Within each story, the test tasks (e.g., T034–T037) are parallel; frontend component tasks (T041/T042, T060) are parallel with backend use-cases until the wiring task.

## Parallel Example: User Story 1 tests

```text
# Launch US1 test-first tasks together (all must fail before implementation):
T034  Rust contract test       (src-tauri/tests/us1_icons.rs)
T035  deckforge unit test      (crates/deckforge/tests/icons.rs)
T036  Storybook stories        (frontend/src/components/StyleOption.stories.tsx)
T037  Playwright flow          (frontend/tests/us1-style-choice.spec.ts)
```

## Implementation Strategy

### MVP First (User Story 1 only)

1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Complete Phase 3 (US1): generate and choose a suit-icon style.
3. **STOP and VALIDATE** US1 independently (fixtures + manual run).
4. This is already a usable, demoable slice — the owner can produce suit iconography they love.

### Incremental Delivery

1. Setup + Foundational → seams ready.
2. US1 → choose icons (MVP) → validate.
3. US2 → style guide → validate.
4. US3 → reveal + approve + persist → validate full feature.
5. Polish → end-to-end + constitution hardening.

## Notes

- [P] = different files, no dependency on an incomplete task.
- Verify each test fails before implementing (Principle VII).
- Everything runs on the FakeProvider + fixed seed; real generation is opt-in via config/env.
- Prompt composition: T038, T049, and T057 depend on T070 (`compose_image_prompt`); iconography
  rules are authoritative over deck-style text in every image prompt (FR-019).
- Failure-handling (T071) and provenance assembly (T072) patterns established for US1/US3 apply
  to the US2 generation step as well.
- Commit after each task or logical group; keep personal data out of commits and logs.
