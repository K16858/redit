pub mod highlighter;
pub use highlighter::{HighlightAnnotation, HighlightState, Highlighter};

pub mod config;
pub use config::{LanguageConfig, default_rust_config, merge_config};

pub mod config_file;
pub use config_file::{BracketConfigFile, ColorRgb, ColorsConfigFile, ConfigFile, RustConfigFile};

pub mod rust;

pub mod registry;
pub use registry::HighlighterRegistry;
