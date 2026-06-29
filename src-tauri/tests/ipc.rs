//! Headless IPC integration: drive the real Tauri `invoke` path through the MockRuntime
//! (no window, no display). Proves the `#[tauri::command]` wrappers + invoke_handler wiring
//! deserialize args, call the core, and serialize responses per contracts/ipc-commands.md.

use deckforge::DeckForge;
use providers::FakeProvider;
use storage::FsStore;
use tauri::ipc::{CallbackFn, InvokeBody};
use tauri::test::{get_ipc_response, mock_builder, INVOKE_KEY};
use tauri::webview::InvokeRequest;
use tauri::WebviewWindowBuilder;

type MockWebview = tauri::WebviewWindow<tauri::test::MockRuntime>;

fn forge(dir: &std::path::Path) -> DeckForge {
    DeckForge::new(
        Box::new(FakeProvider::new()),
        Box::new(FakeProvider::new()),
        Box::new(FsStore::new(dir)),
    )
}

fn invoke(
    webview: &MockWebview,
    cmd: &str,
    args: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    get_ipc_response(
        webview,
        InvokeRequest {
            cmd: cmd.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            // The local app origin on Linux (custom `tauri://` scheme) so the ACL treats this
            // as a local invoke, matching the default capability.
            url: "tauri://localhost".parse().unwrap(),
            body: InvokeBody::Json(args),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<serde_json::Value>().unwrap())
}

#[test]
fn full_journey_over_real_ipc() {
    let dir = tempfile::tempdir().unwrap();
    let app = deckforge_app_lib::build_app(mock_builder(), forge(dir.path()));
    let webview = WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    // No active deck initially.
    assert!(invoke(&webview, "get_active_deck", serde_json::json!({}))
        .unwrap()
        .is_null());

    // US1 — generate (camelCase args map to snake_case params through Tauri).
    let res = invoke(
        &webview,
        "generate_icon_styles",
        serde_json::json!({
            "sessionId": null,
            "deckStyleText": "indigo and gold",
            "count": 3
        }),
    )
    .unwrap();
    let session_id = res["sessionId"].as_str().unwrap().to_string();
    let options = res["styleOptions"].as_array().unwrap();
    assert!(options.len() >= 3);
    let option_id = options[0]["id"].as_str().unwrap().to_string();

    // get_asset returns a base64 PNG payload.
    let icon_key = options[0]["suitIcons"][0]["imageKey"].as_str().unwrap();
    let asset = invoke(
        &webview,
        "get_asset",
        serde_json::json!({ "key": icon_key }),
    )
    .unwrap();
    assert_eq!(asset["mime"], "image/png");
    assert!(!asset["base64"].as_str().unwrap().is_empty());

    // US1→US3 — select, style guide, sample, approve.
    invoke(
        &webview,
        "select_icon_style",
        serde_json::json!({ "sessionId": session_id, "styleOptionId": option_id }),
    )
    .unwrap();
    let guide = invoke(
        &webview,
        "generate_style_guide",
        serde_json::json!({ "sessionId": session_id }),
    )
    .unwrap();
    assert!(!guide["shaderAreas"].as_array().unwrap().is_empty());

    let sample = invoke(
        &webview,
        "compose_sample_card",
        serde_json::json!({ "sessionId": session_id }),
    )
    .unwrap();
    assert_eq!(sample["approval"], "pending");
    // Always a court card (clarification 2026-06-28).
    let rank = sample["courtCard"]["rank"].as_str().unwrap();
    assert!(["page", "knight", "queen", "king"].contains(&rank));

    let deck = invoke(
        &webview,
        "approve_deck",
        serde_json::json!({ "sessionId": session_id }),
    )
    .unwrap();
    assert_eq!(deck["active"], true);

    // The active deck now loads back over IPC (SC-003/004).
    let active = invoke(&webview, "get_active_deck", serde_json::json!({})).unwrap();
    assert_eq!(active["id"], deck["id"]);
}

#[test]
fn errors_map_to_contract_codes_over_ipc() {
    let dir = tempfile::tempdir().unwrap();
    let app = deckforge_app_lib::build_app(mock_builder(), forge(dir.path()));
    let webview = WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    let err = invoke(
        &webview,
        "generate_style_guide",
        serde_json::json!({ "sessionId": "does-not-exist" }),
    )
    .unwrap_err();
    assert_eq!(err["code"], "unknown_session");
    assert_eq!(err["retryable"], false);
}
