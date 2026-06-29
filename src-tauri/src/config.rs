//! Construct the provider + storage seams from configuration/environment (Principle I/III/VI).
//!
//! Defaults to the offline `FakeProvider` and the local filesystem so a fresh checkout runs
//! end-to-end with no credentials. Real generation is opt-in via env; secrets come from the
//! environment only and are never logged.

use std::path::PathBuf;

use deckforge::DeckForge;
use providers::{FakeProvider, ImageProvider, TextProvider};
use storage::FsStore;

/// Resolve the app-data base directory where deck bundles live. The storage crate stays
/// path-agnostic; the host decides the base.
pub fn app_data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("DECKFORGE_DATA_DIR") {
        return PathBuf::from(dir);
    }
    // XDG-ish default; falls back to a local directory.
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|_| PathBuf::from("."));
    base.join("deckforge")
}

/// Build a `DeckForge` from configuration. Today the only provider is the deterministic
/// fake; selecting a real provider here (behind the same traits) is the single change
/// needed to enable real generation — no domain/render code changes (Principle I).
pub fn build_deckforge() -> DeckForge {
    let text: Box<dyn TextProvider> = make_text_provider();
    let image: Box<dyn ImageProvider> = make_image_provider();
    let store = Box::new(FsStore::new(app_data_dir()));
    DeckForge::new(text, image, store)
}

fn make_text_provider() -> Box<dyn TextProvider> {
    match std::env::var("DECKFORGE_PROVIDER").ok().as_deref() {
        // Future: Some("acme") => Box::new(AcmeHttpProvider::from_env()),
        _ => Box::new(FakeProvider::new()),
    }
}

fn make_image_provider() -> Box<dyn ImageProvider> {
    match std::env::var("DECKFORGE_PROVIDER").ok().as_deref() {
        _ => Box::new(FakeProvider::new()),
    }
}
