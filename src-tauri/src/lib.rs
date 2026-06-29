//! Desktop shell for the personal tarot deck. The thinnest possible Tauri 2 layer: it builds
//! the core via `app_core` and registers the command wrappers. All logic is in `app-core`.

mod commands;

/// Build the core and run the Tauri application.
pub fn run() {
    let forge = app_core::build_deckforge();

    tauri::Builder::default()
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
        .run(tauri::generate_context!())
        .expect("error while running the deckforge application");
}
