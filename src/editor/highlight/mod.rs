pub mod highlighter;
pub use highlighter::{HighlightAnnotation, Highlighter};

pub trait Highlighter: Send + Sync {
    fn highlight_line(&self, line: &str, line_idx: usize) -> Vec<HighlightAnnotation>;
    fn language_name(&self) -> &str;
}
