pub struct LanguageConfig {
    pub keywords: &'static [&'static str],
    pub primitive_types: &'static [&'static str],
    pub line_comment_start: &'static str,
    pub block_comment_start: &'static str,
    pub block_comment_end: &'static str,
}

pub const RUST_CONFIG: LanguageConfig = LanguageConfig {
    keywords: &["fn", "let", "mut", "if", "else", "for", "while", "match"],
    primitive_types: &[
        "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
    ],
    line_comment_start: "//",
    block_comment_start: "/*",
    block_comment_end: "*/",
};
