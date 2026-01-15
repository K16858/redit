use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct ColorsConfigFile {
    pub colors: Option<ColorsConfig>,
}

#[derive(Deserialize)]
pub struct ColorsConfig {
    pub keyword: Option<ColorRgb>,
    pub number: Option<ColorRgb>,
    pub type_name: Option<ColorRgb>,
    pub primitive_type: Option<ColorRgb>,
    pub string: Option<ColorRgb>,
    pub comment: Option<ColorRgb>,
    pub brackets: Option<Vec<ColorRgb>>,
}

#[derive(Deserialize)]
pub struct LanguageConfigFile {
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
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    #[allow(dead_code)]
    IoError(std::io::Error),
    #[allow(dead_code)]
    ParseError(toml::de::Error),
}

pub fn load_colors_config(custom_path: Option<&Path>) -> Result<ColorsConfigFile, ConfigError> {
    let config_path = if let Some(path) = custom_path {
        path.to_path_buf()
    } else {
        get_config_dir()?.join("colors.toml")
    };

    if !config_path.exists() {
        return Err(ConfigError::FileNotFound);
    }

    let contents = fs::read_to_string(&config_path).map_err(ConfigError::IoError)?;
    let config: ColorsConfigFile = toml::from_str(&contents).map_err(ConfigError::ParseError)?;

    Ok(config)
}

pub fn load_language_config(
    language: &str,
    custom_path: Option<&Path>,
) -> Result<LanguageConfigFile, ConfigError> {
    let config_path = if let Some(path) = custom_path {
        path.to_path_buf()
    } else {
        get_config_dir()?
            .join("languages")
            .join(format!("{language}.toml"))
    };

    if !config_path.exists() {
        return Err(ConfigError::FileNotFound);
    }

    let contents = fs::read_to_string(&config_path).map_err(ConfigError::IoError)?;
    let config: LanguageConfigFile = toml::from_str(&contents).map_err(ConfigError::ParseError)?;

    Ok(config)
}

fn get_config_dir() -> Result<PathBuf, ConfigError> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("den"))
            .map_err(|_| ConfigError::FileNotFound)
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config").join("den"))
            .map_err(|_| ConfigError::FileNotFound)
    }
}
