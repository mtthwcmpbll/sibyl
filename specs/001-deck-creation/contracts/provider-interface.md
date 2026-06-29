# Contract: Provider-Agnostic AI Interface

The single seam through which all LLM/image generation flows (constitution Principle I). No
vendor SDK type, model id, endpoint, or provider-specific request/response shape may appear
outside an adapter implementing these traits.

## Traits (Rust, in `crates/providers`)

```rust
/// Text generation (e.g., deriving the style-guide prompt from a chosen style).
#[async_trait]
pub trait TextProvider: Send + Sync {
    async fn complete(&self, req: TextRequest) -> Result<TextResponse, ProviderError>;
    /// Stable identifier (provider/model) for provenance — never a secret.
    fn id(&self) -> ProviderId;
}

/// Image generation (suit icons, style-guide art, just-in-time card imagery).
#[async_trait]
pub trait ImageProvider: Send + Sync {
    async fn generate(&self, req: ImageRequest) -> Result<ImageBytes, ProviderError>;
    fn id(&self) -> ProviderId;
}
```

### Request/response shapes (provider-neutral)

- `TextRequest { system: String, prompt: String, seed: u64, params: GenParams }`
- `TextResponse { text: String }`
- `ImageRequest { prompt: String, seed: u64, size: Size, params: GenParams }`
- `ImageBytes { mime: String, bytes: Vec<u8> }`
- `GenParams` — a neutral bag (e.g., temperature, count) mapped *inside* the adapter to vendor
  params. No vendor field names leak out.
- `ProviderError { code: ProviderErrorCode, message: String, retryable: bool }` where
  `ProviderErrorCode ∈ { Unavailable, RateLimited, Timeout, BadResponse, Auth }`.

## Implementations

### `FakeProvider` (default for tests + local dev)

- Deterministic: output is a pure function of `(prompt, seed)` (and `size` for images).
  Image bytes are procedurally generated (e.g., an SVG/PNG keyed by `hash(prompt, seed)`), so
  every run on every platform yields identical output.
- No network, no credentials. This is the substrate the whole pipeline and all tests run on
  (Principles V, VII).
- `id()` returns a stable `"fake"` provider id.

### HTTP adapter (real generation)

- One adapter wrapping a real text+image generation HTTP API. Provider + model selected by
  **configuration**; credentials from **environment only** (never logged, never committed —
  Principle VI).
- Maps neutral requests → vendor payloads and vendor responses → neutral types entirely within
  the adapter.
- Surfaces failures as `ProviderError { retryable }` so the wizard can offer retry (FR-017).

## Prompt composition (per-artifact rules, authoritative)

Image providers expose only a single `prompt` (no system channel). Each generated artifact
type has its OWN app-defined rules file (suit icons, card border, card back, background
imagery, flourish — under `crates/sibyl/resources/prompts/`). The `sibyl` layer builds the
image prompt with `build_image_prompt(rules, style, subject)`, which **prepends** the
artifact's rules, then the owner's deck style as subordinate direction, then the subject.
Rules take precedence (FR-019); the composed string is passed as `ImageRequest.prompt` and
recorded in provenance (Principles V/VI). There is exactly one LLM call per artifact (the
image generation itself) — no separate prompt-composition call.

## Selection & configuration

- The active provider(s) are constructed in the Tauri shell (`src-tauri`) from configuration
  and injected into `sibyl`; domain/render/orchestration depend only on the traits.
- Default configuration uses `FakeProvider` so a fresh checkout runs end-to-end offline.

## Contract tests (written first)

- A trait-level test suite runs against `FakeProvider`: determinism (same input+seed → same
  bytes), error mapping, and `id()` stability.
- An adapter conformance test (recorded/replayed or behind a feature flag) asserts the HTTP
  adapter maps to the same neutral contract — no vendor type escapes.
- A guard test asserts no vendor crate types are referenced outside the `providers` adapter
  module (seam integrity, Principle I).
