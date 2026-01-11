pub mod highlighter;
pub use highlighter::{HighlightAnnotation, HighlightState, Highlighter};

pub mod config;
pub use config::{LanguageConfig, RUST_CONFIG};

pub mod rust;

pub mod registry;
pub use registry::HighlighterRegistry;
