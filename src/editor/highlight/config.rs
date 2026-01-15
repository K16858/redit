#[derive(Clone)]
pub struct BracketConfig {
    pub open: char,
    pub close: char,
    pub color_offset: usize,
}

#[derive(Clone)]
pub struct LanguageConfig {
    pub keywords: Vec<String>,
    pub primitive_types: Vec<String>,
    pub line_comment_start: String,
    pub block_comment_start: String,
    pub block_comment_end: String,
    pub brackets: Vec<BracketConfig>,
}

pub fn default_rust_config() -> LanguageConfig {
    #[cfg(debug_assertions)]
    {
        // Debug build: try to load from docs/examples/default.toml
        use std::path::Path;
        if let Ok(config_file) = load_config_file(Some(Path::new("docs/examples/default.toml"))) {
            if let Some(rust_config) = config_file.rust {
                return merge_config(&hardcoded_rust_config(), Some(&rust_config));
            }
        }
    }

    // Release build or fallback: use hardcoded values
    hardcoded_rust_config()
}

fn hardcoded_rust_config() -> LanguageConfig {
    LanguageConfig {
        keywords: vec![
            "fn".to_string(),
            "let".to_string(),
            "mut".to_string(),
            "if".to_string(),
            "else".to_string(),
            "for".to_string(),
            "while".to_string(),
            "match".to_string(),
        ],
        primitive_types: vec![
            "i8".to_string(),
            "i16".to_string(),
            "i32".to_string(),
            "i64".to_string(),
            "i128".to_string(),
            "u8".to_string(),
            "u16".to_string(),
            "u32".to_string(),
            "u64".to_string(),
            "u128".to_string(),
            "f32".to_string(),
            "f64".to_string(),
        ],
        line_comment_start: "//".to_string(),
        block_comment_start: "/*".to_string(),
        block_comment_end: "*/".to_string(),
        brackets: vec![
            BracketConfig {
                open: '(',
                close: ')',
                color_offset: 0,
            },
            BracketConfig {
                open: '{',
                close: '}',
                color_offset: 1,
            },
            BracketConfig {
                open: '[',
                close: ']',
                color_offset: 2,
            },
        ],
    }
}

use crate::editor::highlight::config_file::{BracketConfigFile, RustConfigFile, load_config_file};

pub fn merge_config(
    default: &LanguageConfig,
    file_config: Option<&RustConfigFile>,
) -> LanguageConfig {
    let file_config = match file_config {
        Some(c) => c,
        None => return default.clone(),
    };

    LanguageConfig {
        keywords: file_config
            .keywords
            .clone()
            .unwrap_or_else(|| default.keywords.clone()),
        primitive_types: file_config
            .primitive_types
            .clone()
            .unwrap_or_else(|| default.primitive_types.clone()),
        line_comment_start: file_config
            .line_comment_start
            .clone()
            .unwrap_or_else(|| default.line_comment_start.clone()),
        block_comment_start: file_config
            .block_comment_start
            .clone()
            .unwrap_or_else(|| default.block_comment_start.clone()),
        block_comment_end: file_config
            .block_comment_end
            .clone()
            .unwrap_or_else(|| default.block_comment_end.clone()),
        brackets: merge_brackets(&default.brackets, file_config.brackets.as_ref()),
    }
}

fn merge_brackets(
    default: &[BracketConfig],
    file_brackets: Option<&Vec<BracketConfigFile>>,
) -> Vec<BracketConfig> {
    let file_brackets = match file_brackets {
        Some(b) => b,
        None => return default.to_vec(),
    };

    file_brackets
        .iter()
        .map(|b| BracketConfig {
            open: b.open.chars().next().unwrap_or('('),
            close: b.close.chars().next().unwrap_or(')'),
            color_offset: b.color_offset.unwrap_or(0),
        })
        .collect()
}
