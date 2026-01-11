use crate::editor::annotated_string::AnnotationType;

pub struct HighlightAnnotation {
    pub annotation_type: AnnotationType,
    pub start: usize,
    pub end: usize,
}

pub trait Highlighter: Send + Sync {
    fn highlight_line(&self, line: &str, line_idx: usize) -> Vec<HighlightAnnotation>;
    fn language_name(&self) -> &str;
}
