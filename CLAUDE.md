<!-- SPECKIT START -->
Active feature: **001-deck-creation** — read the plan and design artifacts before working:

- Plan: `specs/001-deck-creation/plan.md`
- Spec: `specs/001-deck-creation/spec.md`
- Research: `specs/001-deck-creation/research.md`
- Data model: `specs/001-deck-creation/data-model.md`
- Contracts: `specs/001-deck-creation/contracts/` (ipc-commands, deck-bundle, provider-interface)
- Quickstart: `specs/001-deck-creation/quickstart.md`

Stack (per constitution v1.3.1): Rust core (Cargo workspace: domain/providers/render/storage/
deckforge) + Tauri 2 desktop shell (`src-tauri/`) + TypeScript/React/Vite frontend
(`frontend/`). All AI behind provider traits (FakeProvider default); all persistence behind a
Store trait (filesystem default); seeded ChaCha20 RNG + recorded provenance; frontend
decoupled from Tauri via a DeckForgeClient port. Test-first: cargo test, Vitest, Storybook,
Playwright — all on the fake provider + fixed seed.
<!-- SPECKIT END -->
