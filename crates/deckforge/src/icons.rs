use domain::rng::sub_seed;
use domain::{DeckCreationSession, Personal, SessionStatus, StyleOption, Suit, SuitIcon};
use providers::{ImageRequest, Size};

use crate::error::{DeckForgeError, Result};
use crate::prompt::compose_image_prompt;
use crate::DeckForge;

/// Input for generating (or regenerating) a round of suit-icon style options (FR-003/006).
#[derive(Debug, Clone, Default)]
pub struct GenerateIconStyles {
    /// Existing session to regenerate within; `None` starts a new session.
    pub session_id: Option<String>,
    /// Explicit seed for a new session (reproducibility/tests). Derived from inputs if absent.
    pub seed: Option<u64>,
    pub rules: String,
    pub style: String,
    /// Desired option count; clamped to a minimum of 3 (SC-002).
    pub count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconStylesResult {
    pub session_id: String,
    pub style_options: Vec<StyleOption>,
}

impl DeckForge {
    /// Generate a round of cohesive suit-icon style options. Idempotent per (session, round):
    /// the same base seed reproduces the same options; regenerating advances the round.
    pub async fn generate_icon_styles(
        &self,
        input: GenerateIconStyles,
    ) -> Result<IconStylesResult> {
        let mut session = match &input.session_id {
            Some(id) => {
                let mut s = self.load_session(id)?;
                s.round += 1; // regenerate → fresh round (FR-006), inputs preserved
                s
            }
            None => {
                let seed = input
                    .seed
                    .unwrap_or_else(|| sub_seed(0, &format!("{}|{}", input.rules, input.style)));
                let id = format!("sess-{seed:016x}");
                DeckCreationSession::new(
                    id,
                    seed,
                    Personal::from(input.rules.as_str()),
                    Personal::from(input.style.as_str()),
                )
            }
        };

        let rules = session.iconography_rules.as_str().to_string();
        let style = session.deck_style_text.as_str().to_string();
        let round = session.round;
        let round_seed = sub_seed(session.seed, &format!("round-{round}"));
        let count = input.count.max(3);

        let mut options = Vec::with_capacity(count);
        for o in 0..count {
            let option_id = format!("r{round}-opt{o}");
            let option_seed = sub_seed(round_seed, &option_id);

            let base_prompt = compose_image_prompt(
                &*self.text,
                &rules,
                &style,
                &format!("suit-set option {o}"),
                option_seed,
            )
            .await?;

            let mut suit_icons = Vec::with_capacity(Suit::STANDARD.len());
            for suit in Suit::STANDARD {
                let img = self
                    .image
                    .generate(ImageRequest {
                        prompt: format!("{base_prompt} | suit: {suit}"),
                        seed: sub_seed(option_seed, suit.as_str()),
                        size: Size {
                            width: 64,
                            height: 64,
                        },
                        params: Default::default(),
                    })
                    .await?;

                let key = format!("decks/_drafts/{}/icons/{option_id}/{suit}.png", session.id);
                self.store.put(&key, &img.bytes)?;
                suit_icons.push(SuitIcon {
                    suit,
                    image_key: key,
                });
            }

            options.push(StyleOption {
                id: option_id,
                label: format!("Option {}", (b'A' + o as u8) as char),
                suit_icons,
                prompt_used: base_prompt,
            });
        }

        session.style_options = options.clone();
        session.chosen_style_option_id = None;
        session.status = SessionStatus::ChoosingStyle;
        self.save_session(&session)?;

        Ok(IconStylesResult {
            session_id: session.id,
            style_options: options,
        })
    }

    /// Record the owner's chosen style option (FR-005).
    pub fn select_icon_style(&self, session_id: &str, option_id: &str) -> Result<()> {
        let mut session = self.load_session(session_id)?;
        let exists = session.style_options.iter().any(|o| o.id == option_id);
        if !exists {
            return Err(DeckForgeError::UnknownOption(option_id.to_string()));
        }
        session.chosen_style_option_id = Some(option_id.to_string());
        session.status = SessionStatus::StyleGuide;
        self.save_session(&session)?;
        Ok(())
    }
}
