//! Desktop shell for the personal tarot deck. The thinnest possible Tauri 2 layer: it builds
//! the core via `app_core` and registers the command wrappers. All logic is in `app-core`.

pub mod commands;

use sibyl::Sibyl;
use tauri::{App, Builder, Runtime};

/// Assemble the Tauri app: manage the `Sibyl` core and register the IPC commands. Generic
/// over the runtime so tests can build it on the `MockRuntime` and exercise real `invoke`s.
pub fn build_app<R: Runtime>(builder: Builder<R>, forge: Sibyl) -> App<R> {
    builder
        .manage(forge)
        .invoke_handler(tauri::generate_handler![
            commands::generate_icon_styles,
            commands::select_icon_style,
            commands::generate_style_guide,
            commands::compose_sample_card,
            commands::get_asset,
            commands::approve_deck,
            commands::reject_and_restart,
            commands::get_active_deck,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build the sibyl application")
}

/// Build the core from configuration and run the desktop application.
pub fn run() {
    let forge = app_core::build_sibyl();
    build_app(tauri::Builder::default(), forge).run(|_handle, _event| {});
}
