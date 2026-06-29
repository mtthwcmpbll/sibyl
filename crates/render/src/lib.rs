//! Canonical CPU compositing for cards (constitution Principles IV/V).
//!
//! This is the *canonical* render layer: deterministic, headless, byte-stable. The juicy
//! animated/shader presentation lives in the frontend on top of these images, never here.

pub mod compositor;

pub use compositor::{compose, Color, Layer, RenderError};
