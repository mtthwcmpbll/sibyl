fn main() {
    // Autogenerate `allow-*`/`deny-*` permissions for our app commands so a capability can
    // grant them to the main window (Tauri 2 ACL). Without this the frontend's invokes are
    // denied.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "generate_icon_styles",
            "select_icon_style",
            "generate_style_guide",
            "compose_sample_card",
            "get_asset",
            "approve_deck",
            "reject_and_restart",
            "get_active_deck",
        ]),
    ))
    .expect("failed to run tauri-build");
}
