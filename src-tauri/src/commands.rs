//! Frontend ↔ core IPC commands (see specs/001-deck-creation/contracts/ipc-commands.md).
//!
//! Each command is a thin wrapper over a `deckforge` use-case. Errors are returned as a
//! structured `{ code, message, retryable }` so the wizard can offer retry (FR-017).

use base64::Engine;
use deckforge::{DeckForge, GenerateIconStyles, IconStylesResult, RestartInputs};
use domain::{CardStyleGuide, Deck, SampleCard};
use serde::Serialize;

#[derive(Debug, Serialize)]
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

type CmdResult<T> = Result<T, CommandError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    pub key: String,
    pub mime: String,
    pub base64: String,
}

#[tauri::command]
pub async fn generate_icon_styles(
    state: tauri::State<'_, DeckForge>,
    session_id: Option<String>,
    iconography_rules: String,
    deck_style_text: String,
    count: Option<usize>,
) -> CmdResult<IconStylesResult> {
    let out = state
        .generate_icon_styles(GenerateIconStyles {
            session_id,
            seed: None,
            rules: iconography_rules,
            style: deck_style_text,
            count: count.unwrap_or(3),
        })
        .await?;
    Ok(out)
}

#[tauri::command]
pub async fn select_icon_style(
    state: tauri::State<'_, DeckForge>,
    session_id: String,
    style_option_id: String,
) -> CmdResult<()> {
    state.select_icon_style(&session_id, &style_option_id)?;
    Ok(())
}

#[tauri::command]
pub async fn generate_style_guide(
    state: tauri::State<'_, DeckForge>,
    session_id: String,
) -> CmdResult<CardStyleGuide> {
    Ok(state.generate_style_guide(&session_id).await?)
}

#[tauri::command]
pub async fn compose_sample_card(
    state: tauri::State<'_, DeckForge>,
    session_id: String,
) -> CmdResult<SampleCard> {
    Ok(state.compose_sample_card(&session_id).await?)
}

#[tauri::command]
pub async fn get_asset(state: tauri::State<'_, DeckForge>, key: String) -> CmdResult<AssetResponse> {
    let bytes = state.get_asset(&key)?;
    let mime = if key.ends_with(".png") {
        "image/png"
    } else {
        "application/octet-stream"
    };
    Ok(AssetResponse {
        base64: base64::engine::general_purpose::STANDARD.encode(&bytes),
        mime: mime.to_string(),
        key,
    })
}

#[tauri::command]
pub async fn approve_deck(
    state: tauri::State<'_, DeckForge>,
    session_id: String,
) -> CmdResult<Deck> {
    Ok(state.approve_deck(&session_id)?)
}

#[tauri::command]
pub async fn reject_and_restart(
    state: tauri::State<'_, DeckForge>,
    session_id: String,
) -> CmdResult<RestartInputs> {
    Ok(state.reject_and_restart(&session_id)?)
}

#[tauri::command]
pub async fn get_active_deck(state: tauri::State<'_, DeckForge>) -> CmdResult<Option<Deck>> {
    Ok(state.get_active_deck()?)
}

#[cfg(test)]
mod tests {
    //! Contract: the command layer maps core errors to the stable codes/retryable flags in
    //! contracts/ipc-commands.md (T027/T034/T044/T052). The use-case behavior itself is
    //! verified headlessly in the `deckforge` crate's contract tests.
    use super::CommandError;
    use deckforge::DeckForgeError;

    #[test]
    fn error_codes_match_the_ipc_contract() {
        let cases = [
            (
                DeckForgeError::UnknownSession("s".into()),
                "unknown_session",
                false,
            ),
            (
                DeckForgeError::UnknownOption("o".into()),
                "unknown_option",
                false,
            ),
            (DeckForgeError::NoStyleSelected, "no_style_selected", false),
            (DeckForgeError::NoStyleGuide, "no_style_guide", false),
            (
                DeckForgeError::NothingToApprove,
                "nothing_to_approve",
                false,
            ),
            (
                DeckForgeError::Provider {
                    message: "down".into(),
                    retryable: true,
                },
                "provider_failed",
                true,
            ),
        ];
        for (err, code, retryable) in cases {
            let mapped: CommandError = err.into();
            assert_eq!(mapped.code, code);
            assert_eq!(mapped.retryable, retryable);
        }
    }
}
