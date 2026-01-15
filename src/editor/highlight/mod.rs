pub mod highlighter;
pub use highlighter::{HighlightAnnotation, HighlightState, Highlighter};

pub mod config;
pub use config::{LanguageConfig, default_rust_config, merge_config};

pub mod config_file;
pub use config_file::{
    BracketConfigFile, ColorRgb, ColorsConfig, ColorsConfigFile, LanguageConfigFile,
    load_colors_config, load_language_config,
};

pub mod rust;

pub mod registry;
pub use registry::HighlighterRegistry;
