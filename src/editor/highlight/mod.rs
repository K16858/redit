pub mod highlighter;
pub use highlighter::{HighlightAnnotation, HighlightState, Highlighter};

pub mod config;
pub use config::{LanguageConfig, default_rust_config};

pub mod rust;

pub mod registry;
pub use registry::HighlighterRegistry;
