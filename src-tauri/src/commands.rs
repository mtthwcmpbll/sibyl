//! Frontend ↔ core IPC commands (see specs/001-deck-creation/contracts/ipc-commands.md).
//!
//! Each is a one-line `#[tauri::command]` wrapper over `app_core`, whose logic (arg mapping,
//! error→code mapping, asset encoding) is verified headlessly in `crates/app-core/tests`.

use app_core::{AssetResponse, CmdResult};
use domain::{CardStyleGuide, Deck, SampleCard};
use sibyl::{IconStylesResult, RestartInputs, Sibyl};

#[tauri::command]
pub async fn generate_icon_styles(
    state: tauri::State<'_, Sibyl>,
    session_id: Option<String>,
    deck_style_text: String,
    count: Option<usize>,
) -> CmdResult<IconStylesResult> {
    app_core::generate_icon_styles(&state, session_id, deck_style_text, count).await
}

#[tauri::command]
pub async fn select_icon_style(
    state: tauri::State<'_, Sibyl>,
    session_id: String,
    style_option_id: String,
) -> CmdResult<()> {
    app_core::select_icon_style(&state, &session_id, &style_option_id)
}

#[tauri::command]
pub async fn generate_style_guide(
    state: tauri::State<'_, Sibyl>,
    session_id: String,
) -> CmdResult<CardStyleGuide> {
    app_core::generate_style_guide(&state, &session_id).await
}

#[tauri::command]
pub async fn compose_sample_card(
    state: tauri::State<'_, Sibyl>,
    session_id: String,
) -> CmdResult<SampleCard> {
    app_core::compose_sample_card(&state, &session_id).await
}

#[tauri::command]
pub async fn get_asset(state: tauri::State<'_, Sibyl>, key: String) -> CmdResult<AssetResponse> {
    app_core::get_asset(&state, &key)
}

#[tauri::command]
pub async fn approve_deck(state: tauri::State<'_, Sibyl>, session_id: String) -> CmdResult<Deck> {
    app_core::approve_deck(&state, &session_id)
}

#[tauri::command]
pub async fn reject_and_restart(
    state: tauri::State<'_, Sibyl>,
    session_id: String,
) -> CmdResult<RestartInputs> {
    app_core::reject_and_restart(&state, &session_id)
}

#[tauri::command]
pub async fn get_active_deck(state: tauri::State<'_, Sibyl>) -> CmdResult<Option<Deck>> {
    app_core::get_active_deck(&state)
}
