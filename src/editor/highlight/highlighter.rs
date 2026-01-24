use crate::editor::annotated_string::AnnotationType;

pub struct HighlightAnnotation {
    pub annotation_type: AnnotationType,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Default)]
pub struct HighlightState {
    pub in_block_comment: bool,
    pub paren_level: usize,
    pub brace_level: usize,
    pub bracket_level: usize,
    pub in_string: Option<StringType>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    DoubleQuote,
    SingleQuote,
    Backtick,
    TripleDoubleQuote,
    TripleSingleQuote,
}

pub trait Highlighter: Send + Sync {
    fn highlight_line(
        &self,
        line: &str,
        line_idx: usize,
        state: HighlightState,
    ) -> (Vec<HighlightAnnotation>, HighlightState);
    fn language_name(&self) -> &str;
}
