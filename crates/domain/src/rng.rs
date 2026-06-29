use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::court::CourtCardId;

/// A portable, reproducible RNG seeded from a `u64` (constitution Principle V).
/// ChaCha20 is stable across platforms, unlike the thread/std RNGs.
pub fn rng_from_seed(seed: u64) -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(seed)
}

/// Seeded-random court-card selection for the sample card. Reproducible given the seed, yet
/// varied across attempts — serving Surprise without breaking replay.
pub fn select_court_card(seed: u64) -> CourtCardId {
    let mut rng = rng_from_seed(seed);
    let all = CourtCardId::all();
    *all.choose(&mut rng).expect("court card set is never empty")
}

/// Derive a stable per-option/per-suit sub-seed from a session seed and a label, so each
/// generation request varies deterministically.
pub fn sub_seed(seed: u64, label: &str) -> u64 {
    // FNV-1a over the label, mixed with the session seed — stable across versions/platforms.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in label.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash ^ seed
}
