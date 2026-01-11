pub struct LanguageConfig {
    pub keywords: &'static [&'static str],
}

pub const RUST_CONFIG: LanguageConfig = LanguageConfig {
    keywords: &["fn", "let", "mut", "if", "else", "for", "while", "match"],
};
