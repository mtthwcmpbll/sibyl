use serde::{Deserialize, Serialize};

use domain::Deck;

use crate::store::{Result, Store, StoreError};

/// Pointer to the single active deck (`decks/active.json`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivePointer {
    pub active_deck_id: Option<String>,
}

const ACTIVE_KEY: &str = "decks/active.json";

fn manifest_key(deck_id: &str) -> String {
    format!("decks/{deck_id}/manifest.json")
}

/// Read the active-deck pointer (defaults to "none" if absent).
pub fn read_active(store: &dyn Store) -> Result<ActivePointer> {
    if !store.exists(ACTIVE_KEY) {
        return Ok(ActivePointer::default());
    }
    let bytes = store.get(ACTIVE_KEY)?;
    serde_json::from_slice(&bytes).map_err(|e| StoreError::Serde(e.to_string()))
}

/// Persist a deck manifest and (atomically, last-write-wins) mark it the single active deck.
pub fn write_deck_and_activate(store: &dyn Store, deck: &Deck) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(deck).map_err(|e| StoreError::Serde(e.to_string()))?;
    store.put(&manifest_key(&deck.id), &bytes)?;

    let pointer = ActivePointer {
        active_deck_id: Some(deck.id.clone()),
    };
    let pbytes =
        serde_json::to_vec_pretty(&pointer).map_err(|e| StoreError::Serde(e.to_string()))?;
    store.put(ACTIVE_KEY, &pbytes)
}

/// Load the active deck, if any (used on app open and by future features).
pub fn read_active_deck(store: &dyn Store) -> Result<Option<Deck>> {
    let pointer = read_active(store)?;
    let Some(id) = pointer.active_deck_id else {
        return Ok(None);
    };
    let bytes = store.get(&manifest_key(&id))?;
    let deck = serde_json::from_slice(&bytes).map_err(|e| StoreError::Serde(e.to_string()))?;
    Ok(Some(deck))
}
