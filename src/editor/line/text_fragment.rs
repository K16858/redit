use super::GraphemeWidth;

#[derive(Clone)]
pub struct TextFragment {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
    pub start: usize,
}
