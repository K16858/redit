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
    Bracket0,
    Bracket1,
    Bracket2,
    Bracket3,
}
