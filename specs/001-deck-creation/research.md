# Phase 0 Research: Deck Creation

All Technical Context unknowns are resolved below. The bound stack (Rust, Tauri, TypeScript,
portable frontend, test-first toolchain) comes from the constitution; remaining choices are
recorded here with rationale and alternatives.

## 1. Desktop framework version — Tauri 2.x

- **Decision**: Tauri 2.x.
- **Rationale**: Constitution binds Tauri. v2 is current, has a stable command/IPC model, and
  keeps the door open to the shape-A mobile path discussed for the project (native shell over
  a Rust core) without re-platforming. Rust core + web frontend matches the "own identity +
  web juice" goal.
- **Alternatives**: Tauri 1.x (older, no mobile path) — rejected. Electron — rejected (not the
  bound stack; heavier; no Rust core).

## 2. Frontend framework — React 18 + Vite 5 + TypeScript

- **Decision**: React 18 with Vite 5, TypeScript strict mode.
- **Rationale**: Best-supported combination for the bound test toolchain — Vitest is native to
  Vite, Storybook 8 has first-class React+Vite support, Playwright is framework-agnostic.
  Large ecosystem for the animated/shader presentation layer (Canvas/WebGL/WebGPU embed
  cleanly in a React component). Reversible: the wizard logic lives in a framework-agnostic
  TS state machine, so a later swap touches views, not core logic.
- **Alternatives**: Svelte (lighter, great animations) and SolidJS (fast) — viable but smaller
  Storybook/tooling ecosystems; deferred. Vanilla TS — rejected (component testing/documentation
  story weaker).

## 3. Provider-agnostic AI seam — traits + FakeProvider + one HTTP adapter

- **Decision**: Define `TextProvider` and `ImageProvider` traits in `providers`. Ship a
  deterministic `FakeProvider` (default for tests/dev) and one configurable HTTP adapter for
  real generation. No vendor type, model id, or endpoint escapes the adapter (Principle I).
- **Rationale**: Satisfies provider-agnosticism and the headless-test requirement. The
  `FakeProvider` returns deterministic placeholder imagery derived from a hash of the prompt +
  seed, so the full pipeline runs with no network (Principles V, VII). Provider + model are
  selected by configuration; credentials come from environment only.
- **FakeProvider determinism**: image bytes are a function of `(prompt, seed)` — e.g., a
  procedurally generated SVG/PNG keyed by the hash — guaranteeing identical output across runs
  and platforms for tests.
- **Alternatives**: Hard-coding a single vendor SDK — rejected (violates Principle I). Mocking
  at the HTTP layer only — rejected (a first-class fake is cleaner and reusable in dev).

## 3a. Prompt composition — per-artifact rules are authoritative

- **Decision**: Each generated artifact type has its own app-defined rules file
  (`crates/sibyl/resources/prompts/{suit_icons,card_border,card_back,background_image,flourish}.md`).
  A pure `build_image_prompt(rules, style, subject)` helper **prepends** the artifact's rules,
  then the owner's deck style (subordinate), then the subject, yielding the final
  `ImageRequest.prompt`; rules win on conflict. There is exactly one LLM call per artifact (the
  image generation); the composed prompt is recorded in provenance. (Earlier this routed
  through a `TextProvider` composition call, since removed as redundant.)
- **Rationale**: Honors the owner's intent that the rules are a "system prompt component"
  distinct from style. Image APIs typically expose only a single prompt (no system channel),
  so the separation is enforced at composition time rather than relying on the image model
  (Principle I keeps this provider-neutral; Principles V/VI require recording the composed
  prompt that was actually sent).
- **Alternatives**: Concatenating rules + style flat into one prompt — rejected (loses the
  authority of the rules, which the owner explicitly requested). Relying on an image model's
  own system field — rejected (not portable across providers).

## 4. Canonical rendering / compositing — tiny-skia + resvg + image + ab_glyph

- **Decision**: CPU compositing pipeline in the `render` crate: `image` for decode/encode,
  `tiny-skia` for layered 2D compositing, `resvg`/`usvg` for vector frames/sigils/flourishes,
  `ab_glyph` for typography.
- **Rationale**: Deterministic, headless, cross-platform, no GPU/driver dependency — the
  "canonical render" layer from the project's design discussions (Principle V). Produces the
  card front/back PNGs that the frontend then animates. Scales to wallpaper resolution later
  without changing the pipeline.
- **Alternatives**: GPU rendering (wgpu) for the canonical image — rejected (non-deterministic
  across drivers, heavier, conflicts with reproducibility). The GPU/shader work stays in the
  frontend *presentation* layer, not the canonical render.

## 5. Reproducibility — ChaCha20 seeded RNG + provenance + stored bytes

- **Decision**: Use `rand_chacha::ChaCha20Rng` seeded per creation session for all randomness
  (court-card selection, any prompt variation ordering). Persist a provenance record (inputs,
  chosen style id, seed, provider/model identifier, asset/style-guide versions) and the actual
  generated image bytes in the bundle.
- **Rationale**: AI generation is not deterministic across providers, so "reproducible" =
  recorded inputs + cached outputs. ChaCha20 is portable and stable across platforms. Storing
  the generated bytes guarantees identical re-display (SC-007).
- **Court-card selection**: choose one of the 16 court cards via the seeded RNG each attempt —
  reproducible given the seed, yet varied across attempts (serves Surprise without breaking
  replay). The chosen card id is recorded in provenance.
- **Alternatives**: `StdRng`/thread RNG — rejected (not guaranteed reproducible across
  versions/platforms). Fixed court card (e.g., always Queen) — rejected as the default;
  randomized-but-recorded better serves the Guiding Experience while staying replayable.

## 6. Prepared-state bundle + storage — Store trait, filesystem default, JSON manifest

- **Decision**: A `Store` trait (get/put/list by key) with a filesystem implementation as the
  default. The deck is a bundle directory: a JSON `manifest` + image assets (icon set,
  style-guide art, sample card front/back) + provenance. An `active-deck` pointer marks the
  approved deck. Drafts are persisted under a draft key so retries/back-navigation don't
  regenerate; only approval sets the active pointer.
- **Rationale**: Matches Principle III (prepared state, swappable storage, filesystem default,
  bakeable bundle). JSON manifest is machine-readable and consumable by a future web/FaaS draw
  service. Desktop bundles live under the OS app-data dir (resolved by the Tauri shell, passed
  into `storage` as a base path — the crate itself stays path-agnostic).
- **Alternatives**: SQLite/embedded DB — rejected for MVP (heavier; a file bundle is simpler,
  inspectable, and directly "bakeable"). TOML manifest — rejected (JSON travels better to web).

## 7. Frontend ↔ core boundary — SibylClient port

- **Decision**: A TypeScript `SibylClient` interface defines all core operations. A
  `TauriSibylClient` implements it via `@tauri-apps/api` `invoke`; a `MockSibylClient`
  (and later an HTTP impl) implements it for Storybook/Playwright/web. No component imports
  Tauri APIs directly.
- **Rationale**: Enforces the frontend-portability constraint and is what makes the UI testable
  headlessly (Principle VII) and reusable as a plain web app (Principle II). The Tauri command
  names/shapes are pinned in `contracts/ipc-commands.md`.
- **Alternatives**: Calling `invoke` throughout components — rejected (couples UI to Tauri,
  breaks portability and browser-based tests).

## 8. Async & progress — tokio + event/stream progress

- **Decision**: Core use-cases are async (`tokio`). Long generation steps emit progress the
  frontend can render; the UI never blocks (work runs off the UI thread in the Rust backend).
- **Rationale**: SC-001/SC-006 require visible feedback and no silent hangs. Tauri's
  command/event model surfaces progress and retryable failures to the wizard.
- **Alternatives**: Synchronous blocking calls — rejected (freezes UI, violates feedback SCs).

## Resolved unknowns summary

| Unknown | Resolution |
|---------|-----------|
| Desktop framework version | Tauri 2.x |
| Frontend framework | React 18 + Vite 5 + TS (strict) |
| AI provider approach | Traits + FakeProvider default + one HTTP adapter |
| Canonical rendering | tiny-skia + resvg + image + ab_glyph (CPU, deterministic) |
| Randomness/repro | ChaCha20 seeded RNG + provenance + stored bytes |
| Court-card selection | Seeded-random among 16 court cards, recorded |
| Storage/bundle | Store trait, filesystem default, JSON manifest bundle |
| FE↔core boundary | SibylClient port (Tauri + mock impls) |
| Async/progress | tokio + Tauri events; non-blocking UI |
