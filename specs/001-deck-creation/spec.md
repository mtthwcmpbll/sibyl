# Feature Specification: Deck Creation

**Feature Branch**: `001-deck-creation`

**Created**: 2026-06-28

**Status**: Draft

**Input**: User description: "let's build the initial desktop application that lets me create my personal deck. The app should use an LLM to generate a new set of suit icons (ie cups, wands, etc) in a few styles and let me choose which speaks to me. I'd like to be able to provide a system prompt component for this generation where I can provide explicit rules for generating suit iconography, and a secondary input that the user can define their deck style in text to be used for the icons. once I've selected an icon style I like, the app should generate a new LLM prompt from the chosen style to create the overall card style guide (card borders and chrome, card back, UV coating areas for animated shaders, flourishes, etc). To wrap up the feature, the app should display a single card face down combining all style info (suit icon, card back and front from style guide, and just-in-time LLM generated card imagery. The app should then flip the card to display it to me and ask for my approval. if I approve it then this becomes my deck and I can use it for future features. If I disapprove, I can start over from the beginning."

## Clarifications

### Session 2026-06-28

- Q: Which card should the approval sample card depict? → A: Always a **court/face card** (Page, Knight, Queen, or King) — these present the richest imagery for judging the future deck. The remaining documented assumptions (standard four suits, three style options, single active deck, optional inputs) are confirmed as-is.

## Guiding Experience

Per the project constitution's first gate, this feature must amplify the three Guiding
Experiences:

- **Surprise**: the face-down → flip reveal and the multiple, varied style options each round
  keep seeing a card non-obvious and alive; generation favors genuine variation over templated
  repetition (determinism is reserved for testing/replay, not the default feel).
- **Unexpected Connections**: the owner's free-text deck-style description and iconography
  rules seed a personal visual language that later draw features weave into spreads; this
  feature establishes the vocabulary those future connections draw on.
- **Inspired by the Visual Identity**: the entire feature exists to establish a coherent,
  evocative visual identity (suit icons + card style guide) so every future card invites
  interpretation rather than merely illustrating — which is why the approval card is a rich
  court/face card.

## User Scenarios & Testing *(mandatory)*

This feature is a one-time **deck creation** journey in the desktop app. It establishes the
owner's personal tarot deck visual identity — the suit iconography and the overall card
style guide — and ends with the owner approving a single composed sample card. An approved
result becomes the owner's active deck, reusable by future features. The journey is a guided
wizard, but each stage is independently testable using fixed inputs.

### User Story 1 - Generate and choose a suit-icon style (Priority: P1)

The owner opens deck creation, provides two text inputs — a set of explicit **iconography
rules** (governing how suit icons should be generated) and a free-text **deck-style
description** — and asks the app to generate suit icons. The app produces several distinct
**style options**, each rendering the full set of suits as one cohesive look. The owner
reviews the options and selects the one that speaks to them.

**Why this priority**: The chosen suit iconography is the seed of the entire deck identity;
nothing downstream can happen without it. On its own it already delivers value — the owner
ends up with a set of suit icons they love.

**Independent Test**: Provide a fixed iconography-rules input and a fixed deck-style
description, run generation, and confirm the owner is shown multiple cohesive style options
and can select exactly one. Fully testable without any later stage.

**Acceptance Scenarios**:

1. **Given** the owner is on the input step, **When** they enter iconography rules and a
   deck-style description and start generation, **Then** the app presents at least three
   distinct style options, each showing every suit in a consistent style.
2. **Given** style options are displayed, **When** the owner selects one option, **Then**
   that option is recorded as the chosen suit iconography and the owner can proceed.
3. **Given** none of the options appeal, **When** the owner chooses to regenerate, **Then**
   the app produces a new set of options, with the owner's prior inputs preserved and
   editable.
4. **Given** generation is running, **When** the owner waits, **Then** the app shows visible
   progress and either presents options or reports a failure it can retry.

---

### User Story 2 - Generate the card style guide from the chosen style (Priority: P2)

Once a suit-icon style is chosen, the app automatically derives a new generation prompt from
that chosen style and produces an overall **card style guide**: card borders and chrome, the
card back, the card front layout, designated areas for animated shader ("UV coating")
effects, and decorative flourishes.

**Why this priority**: The style guide turns a set of icons into a coherent card system and
is required before any card can be composed. It is the bridge between iconography and a
finished card.

**Independent Test**: Provide a fixed "chosen suit-icon style" as input, run style-guide
generation, and confirm the app produces a style guide covering borders/chrome, card back,
front layout, shader/UV areas, and flourishes — derived from the chosen style without asking
the owner to restate it.

**Acceptance Scenarios**:

1. **Given** a chosen suit-icon style, **When** the owner proceeds, **Then** the app
   generates a style guide that includes, at minimum, card border/chrome, a card back, a
   card front layout, designated animated-shader/UV areas, and flourishes.
2. **Given** a chosen style, **When** the style-guide prompt is created, **Then** it is
   derived from the chosen style automatically rather than requiring new owner input.
3. **Given** style-guide generation fails, **When** the failure occurs, **Then** the owner
   is informed and can retry without losing the chosen style.

---

### User Story 3 - Preview, flip, and approve the sample card (Priority: P3)

The app composes a single **sample card** — always a **court/face card** (Page, Knight,
Queen, or King), chosen because its rich figure imagery is the best test of the future deck's
look — combining the chosen suit icon, the card back and front from the style guide, and
just-in-time generated card imagery. It shows the card face down, flips it to reveal the
face, and asks the owner to approve. On approval, the result becomes the owner's active deck
for use by future features. On rejection, the owner can start over from the beginning.

**Why this priority**: This is the moment of truth that converts generated artifacts into a
committed deck identity. It delivers the headline experience (the reveal) and the durable
outcome (an approved deck).

**Independent Test**: Provide fixed chosen-icon and style-guide inputs, compose the sample
card, and confirm it displays face down, flips to reveal, and that approving persists an
active deck while rejecting leaves no active deck and returns to the start.

**Acceptance Scenarios**:

1. **Given** a chosen icon style and a generated style guide, **When** the sample card is
   composed, **Then** it is a court/face card (Page, Knight, Queen, or King) combining the
   suit icon, the style-guide card back and front, and freshly generated card imagery into
   one card.
2. **Given** the sample card is ready, **When** it is presented, **Then** it first appears
   face down and then flips to reveal its face.
3. **Given** the revealed card, **When** the owner approves it, **Then** the chosen icons,
   the style guide, and the inputs that produced them are saved as the owner's active deck,
   available to future features.
4. **Given** the revealed card, **When** the owner rejects it, **Then** no active deck is
   established and the owner is returned to the beginning with prior inputs preserved and
   editable.
5. **Given** the owner closes the app before approving, **When** they reopen it, **Then** no
   unapproved deck has become active.

---

### Edge Cases

- **Generation failure or timeout** at any stage: the owner is informed and can retry; no
  previously entered inputs or chosen selections are lost.
- **Empty inputs**: if the iconography rules or deck-style description are left blank, the app
  proceeds using sensible defaults rather than blocking.
- **No appealing options**: the owner can regenerate style options repeatedly, editing inputs
  between rounds.
- **Non-standard suits**: if the iconography rules specify a different set of suits than the
  standard four, the generated set follows the rules.
- **Replacing an existing deck**: if an active deck already exists, a newly approved deck
  becomes the active deck only upon approval; rejecting leaves the existing deck untouched.
- **Abandoned session**: closing the app mid-journey does not establish a deck; only approval
  does.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST let the owner provide an **iconography rules** input — explicit,
  authoritative rules that govern how suit icons are generated and that constrain every
  icon-generation request.
- **FR-002**: System MUST let the owner provide a free-text **deck-style description** used to
  guide icon generation.
- **FR-003**: System MUST generate suit icons covering every suit of the deck and present them
  as multiple distinct **style options** (default: at least three), each rendering the full
  suit set in one cohesive style.
- **FR-004**: System MUST render every suit within a style option in a visually consistent
  manner so the owner can judge the set as a whole.
- **FR-005**: System MUST let the owner select exactly one style option as their chosen suit
  iconography.
- **FR-006**: System MUST let the owner regenerate style options, preserving and allowing
  edits to their prior inputs between rounds.
- **FR-007**: From the chosen suit-icon style, System MUST automatically derive a generation
  prompt for the card style guide — without requiring the owner to restate the style.
- **FR-008**: System MUST produce a **card style guide** that includes, at minimum: card
  border/chrome, a card back, a card front layout, designated areas for animated
  shader/"UV coating" effects, and decorative flourishes.
- **FR-009**: System MUST compose a single **sample card** that combines the chosen suit
  icon, the card back and front from the style guide, and just-in-time generated card
  imagery. The sample card MUST be a court/face card (Page, Knight, Queen, or King).
- **FR-010**: System MUST first present the sample card face down and then flip it to reveal
  its face.
- **FR-011**: System MUST ask the owner to approve or reject the revealed sample card.
- **FR-012**: On approval, System MUST persist the result as the owner's **active deck** —
  including the chosen suit icons, the style guide, and the inputs/prompts that produced them
  — so future features can use it.
- **FR-013**: On rejection, System MUST let the owner start over from the beginning, with
  prior inputs preserved and editable.
- **FR-014**: System MUST record the provenance of each generated artifact (the iconography
  rules, deck-style description, chosen style, and the identifiers/seed used) so the deck can
  be explained and regenerated.
- **FR-015**: System MUST keep the owner's inputs and the resulting deck stored locally, under
  the owner's control.
- **FR-016**: System MUST show visible progress while any generation step is in flight.
- **FR-017**: System MUST handle generation failures gracefully — informing the owner and
  allowing retry — without discarding already-entered inputs or selections.
- **FR-018**: System MUST treat only an **approved** result as the active deck; unapproved or
  rejected attempts MUST NOT become the active deck.
- **FR-019**: When composing image-generation prompts, System MUST treat the iconography rules
  as authoritative constraints that take precedence over the deck-style description; the
  deck-style description supplies aesthetic direction *within* those rules. The composed prompt
  actually used MUST be recorded in provenance (see FR-014).

### Key Entities

- **Personal Deck (Deck Identity)**: The durable outcome of this feature. References the
  chosen suit-icon set, the card style guide, and the generation provenance; has an active
  status once approved. Reusable by future features.
- **Iconography Rules Input**: Owner-authored explicit rules constraining how suit icons are
  generated.
- **Deck-Style Description**: Owner-authored free-text describing the desired deck style.
- **Style Option (Suit-Icon Set)**: One candidate set rendering all suits in a single cohesive
  style; the owner selects exactly one.
- **Card Style Guide**: The card-system definition derived from the chosen style — border/
  chrome, card back, card front layout, animated-shader/UV areas, and flourishes.
- **Sample Card**: The single composed preview card — always a court/face card (Page, Knight,
  Queen, or King) — combining suit icon + style-guide back/front + just-in-time imagery, with
  face-down/face-up presentation and an approval outcome.
- **Generation Provenance**: The recorded inputs, chosen style, and generation identifiers/
  seed sufficient to explain and regenerate an artifact.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: From a blank start, the owner can complete the journey — generate icons, choose
  a style, generate a style guide, and approve a sample card — in a single sitting, with no
  step leaving the owner without feedback for more than a few seconds.
- **SC-002**: The owner is presented with at least three distinct suit-icon style options to
  choose from in each generation round.
- **SC-003**: 100% of approved decks are still present and reusable when the app is reopened in
  a later session.
- **SC-004**: After approval, reopening the app surfaces the same deck identity (same chosen
  icons and style guide) as was approved.
- **SC-005**: A rejected attempt results in no active deck, and the owner can return to the
  beginning with their previously entered inputs intact.
- **SC-006**: Every generation step either completes or reports a retryable failure — the
  owner never encounters a silent, unrecoverable hang.
- **SC-007**: The approved deck's look is consistent every time it is displayed (the same
  inputs and recorded provenance reproduce the same identity).
- **SC-008**: The sample card visibly begins face down and flips to a revealed face before the
  approval prompt appears.

## Assumptions

- The deck uses the **standard four tarot suits** (Cups, Wands, Swords, Pentacles) unless the
  iconography rules input specifies a different set.
- The default number of style options per generation round is **three** (more is acceptable).
- The app maintains **one active personal deck at a time** (consistent with single-owner use).
- Both text inputs (iconography rules, deck-style description) are **optional**; sensible
  defaults apply when blank.
- The **sample card** shown for approval is always a single court/face card (Page, Knight,
  Queen, or King), chosen for its rich figure imagery. Generating finished imagery for the
  full set of cards is **out of scope** here and happens just-in-time in future features.
- **"Start over from the beginning"** returns the owner to the input step with prior inputs
  preserved and editable.
- Only an **approved** deck becomes the active, reusable deck; rejected or abandoned attempts
  are discarded.
- The owner's inputs and approved deck are stored **locally on the owner's machine**.

## Scope

**In scope**: the two-input suit-icon generation, style selection and regeneration, automatic
style-guide generation from the chosen style, composing a single sample card, the face-down →
flip → approve/reject interaction, persisting an approved deck identity for reuse, and
restarting after rejection.

**Out of scope**: drawing spreads; generating finished imagery for all cards in the deck;
implementing the animated shader effects themselves (this feature only designates where they
go); managing multiple decks; sharing or exporting the deck; any cloud/remote generation.

## Dependencies

- Requires access to an AI capability that can generate both text (prompts) and imagery
  (icons, style-guide art, card imagery). The specific provider is not assumed.
- Relies on local storage for persisting the owner's inputs and approved deck identity.
