use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct ConfigFile {
    pub rust: Option<RustConfigFile>,
    pub colors: Option<ColorsConfigFile>,
}

#[derive(Deserialize)]
pub struct RustConfigFile {
    pub keywords: Option<Vec<String>>,
    pub primitive_types: Option<Vec<String>>,
    pub line_comment_start: Option<String>,
    pub block_comment_start: Option<String>,
    pub block_comment_end: Option<String>,
    pub brackets: Option<Vec<BracketConfigFile>>,
}

#[derive(Deserialize)]
pub struct BracketConfigFile {
    pub open: String,
    pub close: String,
    pub color_offset: Option<usize>,
}

#[derive(Deserialize)]
pub struct ColorsConfigFile {
    pub keyword: Option<ColorRgb>,
    pub number: Option<ColorRgb>,
    pub type_name: Option<ColorRgb>,
    pub primitive_type: Option<ColorRgb>,
    pub string: Option<ColorRgb>,
    pub comment: Option<ColorRgb>,
    pub brackets: Option<Vec<ColorRgb>>,
}

#[derive(Deserialize)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    IoError(std::io::Error),
    ParseError(toml::de::Error),
}

pub fn load_config_file(path: Option<&Path>) -> Result<ConfigFile, ConfigError> {
    let config_path = path.unwrap_or_else(|| Path::new("den.toml"));

    if !config_path.exists() {
        return Err(ConfigError::FileNotFound);
    }

    let contents = fs::read_to_string(config_path).map_err(ConfigError::IoError)?;

    let config: ConfigFile = toml::from_str(&contents).map_err(ConfigError::ParseError)?;

    Ok(config)
}
