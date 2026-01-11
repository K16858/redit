#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Keyword,
    Number,
    Type,
    PrimitiveType,
    String,
    Comment,
}
