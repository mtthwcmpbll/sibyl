//! Tauri-free application/command layer. The desktop shell's IPC commands are thin wrappers
//! over these functions, so the mapping logic is verified headlessly (no webkit required).

use std::path::PathBuf;

use base64::Engine;
use deckforge::{DeckForge, GenerateIconStyles, IconStylesResult, RestartInputs};
use domain::{CardStyleGuide, Deck, SampleCard};
use providers::{AgyCliProvider, ClaudeCliProvider, FakeProvider, ImageProvider, TextProvider};
use serde::Serialize;
use storage::FsStore;

/// Structured error matching contracts/ipc-commands.md `{ code, message, retryable }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl From<deckforge::DeckForgeError> for CommandError {
    fn from(e: deckforge::DeckForgeError) -> Self {
        use deckforge::DeckForgeError as E;
        let code = match &e {
            E::Provider { .. } => "provider_failed",
            E::Storage(_) => "storage_error",
            E::Render(_) => "render_error",
            E::UnknownSession(_) => "unknown_session",
            E::UnknownOption(_) => "unknown_option",
            E::NoStyleSelected => "no_style_selected",
            E::NoStyleGuide => "no_style_guide",
            E::NothingToApprove => "nothing_to_approve",
            E::InvalidState(_) => "invalid_state",
        };
        CommandError {
            code: code.to_string(),
            retryable: e.retryable(),
            message: e.to_string(),
        }
    }
}

pub type CmdResult<T> = Result<T, CommandError>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    pub key: String,
    pub mime: String,
    pub base64: String,
}

// --- Command functions (1:1 with the IPC contract) -----------------------------------------

pub async fn generate_icon_styles(
    forge: &DeckForge,
    session_id: Option<String>,
    deck_style_text: String,
    count: Option<usize>,
) -> CmdResult<IconStylesResult> {
    Ok(forge
        .generate_icon_styles(GenerateIconStyles {
            session_id,
            seed: None,
            style: deck_style_text,
            count: count.unwrap_or(3),
        })
        .await?)
}

pub fn select_icon_style(
    forge: &DeckForge,
    session_id: &str,
    style_option_id: &str,
) -> CmdResult<()> {
    forge.select_icon_style(session_id, style_option_id)?;
    Ok(())
}

pub async fn generate_style_guide(
    forge: &DeckForge,
    session_id: &str,
) -> CmdResult<CardStyleGuide> {
    Ok(forge.generate_style_guide(session_id).await?)
}

pub async fn compose_sample_card(forge: &DeckForge, session_id: &str) -> CmdResult<SampleCard> {
    Ok(forge.compose_sample_card(session_id).await?)
}

pub fn get_asset(forge: &DeckForge, key: &str) -> CmdResult<AssetResponse> {
    let bytes = forge.get_asset(key)?;
    let mime = if key.ends_with(".png") {
        "image/png"
    } else {
        "application/octet-stream"
    };
    Ok(AssetResponse {
        key: key.to_string(),
        mime: mime.to_string(),
        base64: base64::engine::general_purpose::STANDARD.encode(&bytes),
    })
}

pub fn approve_deck(forge: &DeckForge, session_id: &str) -> CmdResult<Deck> {
    Ok(forge.approve_deck(session_id)?)
}

pub fn reject_and_restart(forge: &DeckForge, session_id: &str) -> CmdResult<RestartInputs> {
    Ok(forge.reject_and_restart(session_id)?)
}

pub fn get_active_deck(forge: &DeckForge) -> CmdResult<Option<Deck>> {
    Ok(forge.get_active_deck()?)
}

// --- Construction from configuration/environment -------------------------------------------

/// App-data base directory where deck bundles live (Principle III). Configurable via env.
pub fn app_data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("DECKFORGE_DATA_DIR") {
        return PathBuf::from(dir);
    }
    std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("deckforge")
}

/// Build a `DeckForge` from configuration. Defaults to the offline `FakeProvider` + local
/// filesystem so a fresh checkout runs end-to-end with no credentials (Principle I/II/VI).
pub fn build_deckforge() -> DeckForge {
    DeckForge::new(
        make_text_provider(),
        make_image_provider(),
        Box::new(FsStore::new(app_data_dir())),
    )
}

/// Select a provider by `DECKFORGE_PROVIDER`: `claude` drives the Claude Code CLI and
/// `agy`/`antigravity` drives the Google Antigravity CLI (both vector/SVG art); anything else
/// uses the offline deterministic fake.
fn make_text_provider() -> Box<dyn TextProvider> {
    match std::env::var("DECKFORGE_PROVIDER").ok().as_deref() {
        Some("claude") => Box::new(ClaudeCliProvider::from_env()),
        Some("agy") | Some("antigravity") => Box::new(AgyCliProvider::from_env()),
        _ => Box::new(FakeProvider::new()),
    }
}

fn make_image_provider() -> Box<dyn ImageProvider> {
    match std::env::var("DECKFORGE_PROVIDER").ok().as_deref() {
        Some("claude") => Box::new(ClaudeCliProvider::from_env()),
        Some("agy") | Some("antigravity") => Box::new(AgyCliProvider::from_env()),
        _ => Box::new(FakeProvider::new()),
    }
}
