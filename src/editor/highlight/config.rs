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

pub const RUST_CONFIG: LanguageConfig = LanguageConfig {
    keywords: &["fn", "let", "mut", "if", "else", "for", "while", "match"],
    primitive_types: &[
        "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
    ],
    line_comment_start: "//",
    block_comment_start: "/*",
    block_comment_end: "*/",
    brackets: &[
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
};
