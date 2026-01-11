pub mod highlighter;
pub use highlighter::{HighlightAnnotation, HighlightState, Highlighter};

pub mod rust;

pub mod registry;
pub use registry::HighlighterRegistry;
