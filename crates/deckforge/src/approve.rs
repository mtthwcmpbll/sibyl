use domain::{CardStyleGuide, Deck, FlourishRef, SuitIcon};

use crate::error::{DeckForgeError, Result};
use crate::DeckForge;

/// Inputs returned to the wizard after a rejection so they can be edited and retried (FR-013).
#[derive(Debug, Clone)]
pub struct RestartInputs {
    pub rules: String,
    pub style: String,
}

fn relocate(key: &str, sid: &str, deck_id: &str) -> String {
    let from = format!("decks/_drafts/{sid}/");
    let to = format!("decks/{deck_id}/");
    key.replacen(&from, &to, 1)
}

impl DeckForge {
    fn copy(&self, from: &str, to: &str) -> Result<()> {
        let bytes = self.store.get(from)?;
        self.store.put(to, &bytes)?;
        Ok(())
    }

    /// Approve the sample → promote the draft into a permanent bundle and mark it the single
    /// active deck (FR-012/018). Finalizes provenance (FR-014/T072). Idempotent per session.
    pub fn approve_deck(&self, session_id: &str) -> Result<Deck> {
        let session = self.load_session(session_id)?;
        if session.sample_card.is_none() {
            return Err(DeckForgeError::NothingToApprove);
        }
        let guide = session
            .style_guide
            .clone()
            .ok_or(DeckForgeError::NoStyleGuide)?;
        let chosen = session
            .chosen_option()
            .ok_or(DeckForgeError::NoStyleSelected)?
            .clone();

        let deck_id = format!("deck-{}", session.id);

        // Promote referenced assets out of the draft into the deck bundle, rewriting keys.
        let mut suit_icons = Vec::with_capacity(chosen.suit_icons.len());
        for icon in &chosen.suit_icons {
            let new_key = relocate(&icon.image_key, &session.id, &deck_id);
            self.copy(&icon.image_key, &new_key)?;
            suit_icons.push(SuitIcon {
                suit: icon.suit,
                image_key: new_key,
            });
        }

        let border_chrome_key = relocate(&guide.border_chrome_key, &session.id, &deck_id);
        self.copy(&guide.border_chrome_key, &border_chrome_key)?;
        let card_back_key = relocate(&guide.card_back_key, &session.id, &deck_id);
        self.copy(&guide.card_back_key, &card_back_key)?;
        let mut flourishes = Vec::with_capacity(guide.flourishes.len());
        for f in &guide.flourishes {
            let new_key = relocate(&f.asset_key, &session.id, &deck_id);
            self.copy(&f.asset_key, &new_key)?;
            flourishes.push(FlourishRef {
                asset_key: new_key,
                rect: f.rect,
                rotation: f.rotation,
            });
        }

        let style_guide = CardStyleGuide {
            border_chrome_key,
            card_back_key,
            flourishes,
            ..guide
        };

        // Finalize provenance (FR-014): chosen option recorded; the rest accumulated earlier.
        let mut provenance = session.provenance.clone();
        provenance.chosen_style_option_id = Some(chosen.id.clone());

        let deck = Deck {
            id: deck_id,
            active: true,
            suit_icons,
            style_guide,
            provenance,
        };

        storage::write_deck_and_activate(&*self.store, &deck)?;
        // The approved bundle is self-contained; the draft can be discarded.
        self.store
            .delete_prefix(&format!("decks/_drafts/{}", session.id))?;
        Ok(deck)
    }

    /// Reject the sample and discard the draft, returning the inputs to edit and retry
    /// (FR-013). Any existing active deck is left untouched.
    pub fn reject_and_restart(&self, session_id: &str) -> Result<RestartInputs> {
        let session = self.load_session(session_id)?;
        let inputs = RestartInputs {
            rules: session.iconography_rules.as_str().to_string(),
            style: session.deck_style_text.as_str().to_string(),
        };
        self.store
            .delete_prefix(&format!("decks/_drafts/{}", session.id))?;
        Ok(inputs)
    }

    /// Load the active deck, if any (SC-003/004; used on app open and by future features).
    pub fn get_active_deck(&self) -> Result<Option<Deck>> {
        Ok(storage::read_active_deck(&*self.store)?)
    }
}
