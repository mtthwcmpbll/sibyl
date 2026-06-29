<!--
SYNC IMPACT REPORT
==================
Version change: 1.3.0 → 1.3.1
Bump rationale: PATCH — pinned Vitest as the TypeScript/frontend unit-test runner
  (clarifies the toolchain in Principle VII and Section 2; no new principle or constraint).
  1.3.0: added Principle VII (Test-First Discipline, NON-NEGOTIABLE) and a bound technology
  stack (Rust core, Tauri desktop, TypeScript-over-JS, portable frontend) to Section 2,
  plus a test-first quality gate. Prior amendments: 1.2.0 added the Guiding Experience
  triad; 1.1.0 expanded Principle III into the two-phase Preparation/Draw architecture. No
  principle removed or made backward-incompatible.

Guiding Experience (the project's purpose; added 1.2.0):
  - Surprise
  - Unexpected Connections
  - Inspired by the Visual Identity

Principles defined:
  I.   Provider-Agnostic AI Integration (NON-NEGOTIABLE)
  II.  Local-First, Cloud-Parity
  III. Prepared State, Stateless Draws   (renamed from "Stateless Serverless Design")
  IV.  Coherent Visual Identity
  V.   Reproducible Generation
  VI.  Personal Data Stays Private
  VII. Test-First Discipline (NON-NEGOTIABLE)   (added 1.3.0)

Modified in 1.1.0:
  - Principle III renamed and expanded: one-time Preparation phase vs. stateless Draw
    phase; storage abstraction with local-filesystem default, bakeable prepared-state
    bundle, swappable backend.
  - Section "Technology & Architecture Constraints": asset handling now defines the
    *prepared-state bundle* (profile + identity + assets + templates).
  - Principle VI: prepared-state bundles containing personal data are sensitive in the
    deployment too, not just in the repo.

Added sections:
  - Core Principles (6 principles)
  - Technology & Architecture Constraints (Section 2)
  - Development Workflow & Quality Gates (Section 3)
  - Governance

Removed sections: none

Templates requiring updates:
  ✅ .specify/templates/plan-template.md  — Constitution Check gate is generic; compatible, no edit required
  ✅ .specify/templates/spec-template.md  — uses generic placeholders; compatible, no edit required
  ✅ .specify/templates/tasks-template.md — UPDATED in 1.3.0: tests changed from OPTIONAL to
       REQUIRED-and-test-first to match Principle VII (Test-First Discipline)

Follow-up TODOs: none
-->

# Personal Tarot Deck Constitution

A single-user application that uses an LLM (and image generation) to design and render
a personalized visual tarot deck, drawing spreads on demand and weaving in symbols and
references from the owner's own life.

## Core Principles

### Guiding Experience (NON-NEGOTIABLE)

This deck exists to deliver three experiences. They are the project's reason for being,
and the technical principles that follow (I–VI) exist to serve them. Every feature, prompt,
card, layout, and interaction MUST be designed to amplify all three; a change that is
technically sound but flattens them is a defect.

- **Surprise** — A draw should feel alive and non-obvious. Designs MUST favor genuine
  variation and emergent imagery over predictable, templated, or repetitive results, so
  each spread can still astonish its owner. (This lives in tension with Principle V's
  reproducibility: surprise is the default experience; determinism is a controllable mode
  for testing and replay, never a reason to make draws feel same-y.)
- **Unexpected Connections** — The deck's value is weaving the owner's life into symbols
  in ways they did not see coming. Features MUST seek meaningful, surprising links between
  personal context and tarot symbolism rather than literal or shallow restatement.
- **Inspired by the Visual Identity** — Imagery should provoke reflection and feeling, not
  merely illustrate. The visual identity (Principle IV) MUST be wielded to produce cards
  that invite interpretation and spark insight.

All three are grounded in the personal, intimate nature of fortune-telling: the deck must
read as if it were made for one person, because it is.

### I. Provider-Agnostic AI Integration (NON-NEGOTIABLE)

All access to LLM text generation and AI image generation MUST go through a
project-defined provider interface. No vendor SDK, model name, endpoint, or
provider-specific request/response shape may leak outside the adapter that implements
that interface.

- A new provider MUST be addable by writing one adapter, with zero changes to domain,
  rendering, or orchestration code.
- At least one fake/stub provider MUST exist for local runs and tests so the full
  pipeline works with no external API calls.
- Provider selection and credentials MUST come from configuration/environment, never
  hard-coded.

**Rationale**: The owner explicitly requires no lock-in to any specific AI provider.
An enforced seam keeps the app portable as models and vendors change.

### II. Local-First, Cloud-Parity

The application MUST run end-to-end on the owner's local machine, and the exact same
code paths MUST run unmodified in the serverless target. Differences between local and
cloud MUST be expressed only through configuration, never through branching logic or
separate builds.

- A single command MUST start the app locally and produce a real card/spread.
- Local runs MUST NOT depend on cloud-only services; substitutes (local filesystem,
  fake provider, local object store) MUST be wired through the same interfaces used in
  the cloud.

**Rationale**: The owner wants to test locally and deploy to FaaS; parity prevents
"works on my machine" drift and keeps the test loop fast and cheap.

### III. Prepared State, Stateless Draws

The application MUST be split into two phases that are never conflated:

**Preparation (run once; may be slow, interactive, and AI-assisted)** establishes and
persists all durable state the deck needs — the owner's personal profile, the visual
identity, static visual assets (frames, icons, fonts, sigils), and prompts/templates.
Every prepared artifact MUST be written to storage and addressable by a stable reference.
Preparation MAY run locally and MAY call AI providers.

**Draw (stateless; per request)** takes a request and: draws N cards, generates imagery
for each card, layers on the constant static assets, and lays out the cards — reading all
prepared state from storage by reference. A draw MUST be serviceable by a cold instance
with no local persistent state and no dependency on any prior draw.

The application MUST be deployable to a function-as-a-service / container runtime such as
Google Cloud Run or AWS Lambda, and MUST NOT assume any specific one.

Storage rules:

- All prepared artifacts AND draw outputs MUST be accessed through a single storage
  abstraction; no domain, generation, or compositing module may touch a concrete backend
  directly.
- The DEFAULT backend MUST be the local filesystem, so the owner can run preparation once
  and bake the resulting prepared-state bundle into the deployable image for the cloud.
- The backend MUST be swappable by configuration (e.g., to an object store) with zero
  changes to domain, generation, or compositing code.
- A draw MUST treat prepared state as read-only and the local filesystem as ephemeral,
  writing only transient output (under a temp path or back through the storage
  abstraction).
- Draws MUST fit within typical FaaS limits (bounded execution time, memory, payload);
  the heavy, one-time work belongs in Preparation so draws stay bounded.

**Rationale**: Separating one-time preparation from stateless draws lets the expensive
setup run once and be baked into the deployment, while each spread stays a fast,
self-contained, horizontally scalable function. A filesystem default keeps local use
simple, and the storage seam preserves a clean swap path for the cloud.

### IV. Coherent Visual Identity

The deck MUST have one defined, reusable visual identity — iconography, typography,
color palette, and card frame/layout — captured as explicit, versioned data, not
re-improvised per card.

- The visual identity MUST be established and persisted before or during deck creation,
  and every card MUST be rendered against it so the deck reads as one set.
- Final card imagery MUST be produced by deterministic compositing of AI-generated
  imagery with static/local assets (frames, borders, sigils, type) under the defined
  layout; composition rules MUST live in code/config, not in the image model's prompt
  alone.
- Changes to the visual identity MUST be versioned so previously generated cards remain
  attributable to the identity that produced them.

**Rationale**: A deck is recognizable only if its cards share a system; separating the
identity from per-card content makes that system explicit and consistent — and that shared
system is what lets a card inspire rather than read as generic (see Guiding Experience).

### V. Reproducible Generation

Generation MUST be reproducible for testing and replay. Every source of randomness —
which cards are drawn, prompt variation, and provider sampling — MUST be controllable
via an explicit seed or recorded inputs.

- Card draws MUST use a seedable RNG; given the same seed and inputs, the same cards are
  drawn in the same order.
- Each generated card and spread MUST record the inputs that produced it (seed, card
  identity, prompt inputs, provider/model identifier, asset versions) so it can be
  explained and regenerated.
- Tests MUST be able to exercise the full pipeline deterministically using the fake
  provider and a fixed seed.

**Rationale**: Personalized generation is otherwise unrepeatable and untestable;
recorded inputs make results explainable and the system verifiable.

### VI. Personal Data Stays Private

The deck weaves in references to the owner's own life; that personal context is
sensitive and MUST be handled accordingly.

- Personal context MUST be supplied through configuration/data the owner controls, never
  committed to the repository or embedded in source.
- Personal details MUST NOT be sent to any provider beyond what a given spread requires,
  and what is sent MUST be recorded (per Principle V) so the owner can see it.
- Secrets and personal data MUST be excluded from version control and from logs.
- A prepared-state bundle that contains personal data MUST be treated as sensitive
  wherever it goes — kept out of version control and protected in any deployment image or
  storage backend it is baked into.

**Rationale**: A deck built from one person's life is intimate data; keeping it owner-
controlled and out of the repo and logs respects that.

### VII. Test-First Discipline (NON-NEGOTIABLE)

Tests are written before implementation, always. For every unit of behavior, a failing
test MUST exist and be observed to fail before the code that satisfies it is written;
then make it pass, then refactor (red → green → refactor).

- **Non-UI code (Rust and TypeScript)**: every unit of behavior MUST have unit tests,
  written first (Rust: `cargo test`; TypeScript: **Vitest**). No production logic is
  committed without the test that drove it.
- **Frontend components**: each component MUST have **Storybook** stories covering its
  meaningful states; these serve as both component tests and living documentation.
- **Frontend UI flows**: user-facing behavior and flows MUST be covered by **Playwright**
  UI tests.
- These tests MUST run on the deterministic headless substrate (fake provider + fixed
  seed, per Principles I and V) so they need no network and no real credentials.

**Rationale**: Writing the test first forces each behavior to be specified before it is
built, keeps the design honest, and makes surprise-driven generation verifiable rather
than merely plausible. Storybook and Playwright extend that discipline to the visual layer
where regressions are otherwise easy to miss.

## Technology & Architecture Constraints

- **Technology stack** (binding unless amended by this constitution):
  - **Application core**: Rust.
  - **Desktop app**: Tauri (Rust core + web frontend).
  - **Frontend language**: TypeScript MUST be used over JavaScript wherever a choice
    exists.
  - **Frontend portability**: the frontend MUST NOT depend directly on the Tauri runtime.
    All native capabilities (file save, set-wallpaper, native calls) MUST be reached
    through a thin abstraction that also has a plain-web implementation, so the same
    frontend can run both inside Tauri and as a standalone web app. (This mirrors the
    provider/storage seams of Principles I and III and supports the path where "draw" may
    later move to a service.)
  - **Testing toolchain**: per Principle VII — unit tests for all non-UI code (Rust via
    `cargo test`, TypeScript via Vitest), Storybook for components, Playwright for UI
    flows; all test-first.
- **Single-user scope**: The app serves one owner. Multi-tenant accounts, sharing, and
  authentication beyond protecting the owner's own deployment are out of scope unless
  explicitly added by amendment.
- **Clear seams**: Domain logic (deck, cards, draws), AI providers, asset/compositing,
  and storage MUST be separable modules. Cross-module access goes through interfaces
  named in Principles I–III.
- **Prepared-state bundle**: The personal profile, the visual identity, static assets
  (frames, icons, fonts, sigils), and prompts/templates together form the *prepared-state
  bundle*. They MUST be versioned together and addressed only through the storage
  abstraction, so a complete bundle can be produced once and baked into a deployment.
- **Configuration over code**: Provider choice, credentials, storage backend, target
  runtime, and personal-context location MUST all be configurable without code changes.
- **Dependencies**: Prefer a small, well-understood dependency set; any heavy or
  platform-locking dependency MUST be justified in the plan's Complexity Tracking.

## Development Workflow & Quality Gates

- **Experience check (first gate)**: Every feature spec and plan MUST state how it
  amplifies the three Guiding Experiences — Surprise, Unexpected Connections, and
  inspiration from the visual identity. A feature that cannot show how it serves them, or
  that measurably flattens them, MUST be reworked before any other gate is considered.
- **Constitution Check**: Every `/speckit-plan` MUST pass the Constitution Check gate.
  Violations MUST be recorded in the plan's Complexity Tracking with justification and a
  rejected simpler alternative, or the design MUST change.
- **Test-first gate**: Per Principle VII, no implementation task is complete unless its
  test was written first and is present in the change. Tasks MUST be ordered tests-before-
  implementation; non-UI work has unit tests, components have Storybook stories, UI flows
  have Playwright tests.
- **Pipeline must run headless**: The end-to-end generation pipeline MUST be runnable in
  tests with the fake provider and a fixed seed, with no network and no real
  credentials.
- **Local proof before cloud**: A change to generation, identity, or compositing MUST be
  demonstrated locally (Principle II) before it is considered deployable.
- **Reproducibility checks**: Tests MUST cover that seeded draws are deterministic and
  that recorded inputs are sufficient to regenerate a card/spread.
- **Privacy review**: Any change touching personal context, prompts, logging, or
  outbound provider payloads MUST verify Principle VI (no personal data in repo or logs;
  only required data sent).

## Governance

This constitution supersedes other practices for this project. When guidance conflicts,
the constitution wins.

- **Amendments**: Changes to this document MUST be made by editing this file, with a
  Sync Impact Report describing the change and a version bump per the policy below.
  Dependent templates and guidance docs MUST be reviewed and updated in the same change.
- **Versioning policy** (semantic):
  - MAJOR — removing or redefining a principle in a backward-incompatible way.
  - MINOR — adding a principle or section, or materially expanding guidance.
  - PATCH — clarifications, wording, and non-semantic refinements.
- **Compliance review**: Plans, specs, and tasks MUST be checked against these
  principles. Unjustified complexity or violations block the change until justified in
  Complexity Tracking or resolved.

**Version**: 1.3.1 | **Ratified**: 2026-06-28 | **Last Amended**: 2026-06-28
