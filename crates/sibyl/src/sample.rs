use domain::rng::{select_court_card, sub_seed};
use domain::{Approval, Rect, SampleCard, SessionStatus};
use providers::{ImageRequest, Size};
use render::{compose, Layer};

use crate::error::{Result, SibylError};
use crate::prompt::build_image_prompt;
use crate::{rules, Sibyl};

/// Canvas width for the canonical sample render; height follows the layout aspect.
const CARD_W: u32 = 256;

fn px(rect: Rect, w: u32, h: u32) -> (i64, i64) {
    ((rect.x * w as f32) as i64, (rect.y * h as f32) as i64)
}

impl Sibyl {
    /// Compose the single court/face-card sample (FR-009/010): seeded court-card pick,
    /// just-in-time imagery, and canonical compositing of the front (suit icon + style-guide
    /// front + imagery + chrome) and back. Deterministic given the session seed.
    pub async fn compose_sample_card(&self, session_id: &str) -> Result<SampleCard> {
        let mut session = self.load_session(session_id)?;
        let guide = session
            .style_guide
            .clone()
            .ok_or(SibylError::NoStyleGuide)?;
        let chosen = session
            .chosen_option()
            .ok_or(SibylError::NoStyleSelected)?
            .clone();

        // Seeded court-card selection — reproducible, yet varied across sessions (Surprise).
        let court = select_court_card(sub_seed(session.seed, "court"));

        let style = session.deck_style_text.as_str().to_string();
        // Card imagery prepends the background-image rules (FR-019).
        let imagery_prompt = build_image_prompt(
            rules::BACKGROUND_IMAGE,
            &style,
            &format!("card imagery for the {court}: depict the {court}"),
        );

        let imagery = self
            .image
            .generate(ImageRequest {
                prompt: imagery_prompt.clone(),
                seed: sub_seed(session.seed, &format!("imagery-{}", court.slug())),
                size: Size {
                    width: 64,
                    height: 64,
                },
                params: Default::default(),
            })
            .await?;
        let card_imagery_key = format!("decks/_drafts/{session_id}/sample/imagery.png");
        self.store.put(&card_imagery_key, &imagery.bytes)?;

        // Gather the layers from storage.
        let suit_icon_key = chosen
            .suit_icons
            .iter()
            .find(|i| i.suit == court.suit)
            .map(|i| i.image_key.clone())
            .ok_or_else(|| SibylError::InvalidState("chosen style missing suit".into()))?;
        let icon_bytes = self.store.get(&suit_icon_key)?;
        let chrome_bytes = self.store.get(&guide.border_chrome_key)?;

        let layout = &guide.card_front_layout;
        let h = (CARD_W as f32 / layout.aspect).round() as u32;
        let (ix, iy) = px(layout.card_imagery, CARD_W, h);
        let (sx, sy) = px(layout.suit_icon, CARD_W, h);

        let front = compose(
            CARD_W,
            h,
            [245, 240, 230, 255],
            &[
                Layer {
                    png: &imagery.bytes,
                    x: ix,
                    y: iy,
                },
                Layer {
                    png: &icon_bytes,
                    x: sx,
                    y: sy,
                },
                Layer {
                    png: &chrome_bytes,
                    x: 0,
                    y: 0,
                },
            ],
        )?;
        let front_key = format!("decks/_drafts/{session_id}/sample/front.png");
        self.store.put(&front_key, &front)?;

        let sample = SampleCard {
            court_card: court,
            card_imagery_key,
            front_key,
            back_key: guide.card_back_key.clone(),
            approval: Approval::Pending,
        };

        session
            .provenance
            .prompts
            .insert("sampleImagery".into(), imagery_prompt);
        session.provenance.court_card_shown = Some(court);
        let pid = self.image.id();
        session.provenance.provider_id = pid.provider;
        session.provenance.model_id = pid.model;
        session.sample_card = Some(sample.clone());
        session.status = SessionStatus::Review;
        self.save_session(&session)?;

        Ok(sample)
    }
}
