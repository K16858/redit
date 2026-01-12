pub struct BracketConfig {
    pub open: char,
    pub close: char,
    pub color_offset: usize,
}

pub struct LanguageConfig {
    pub keywords: Vec<String>,
    pub primitive_types: Vec<String>,
    pub line_comment_start: String,
    pub block_comment_start: String,
    pub block_comment_end: String,
    pub brackets: Vec<BracketConfig>,
}

pub fn default_rust_config() -> LanguageConfig {
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
