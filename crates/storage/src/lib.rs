//! Prepared-state persistence behind a swappable [`Store`] (constitution Principle III).
//! The default backend is the local filesystem ([`FsStore`]).

pub mod bundle;
pub mod fs;
pub mod store;

pub use bundle::{read_active, read_active_deck, write_deck_and_activate, ActivePointer};
pub use fs::FsStore;
pub use store::{Store, StoreError};
