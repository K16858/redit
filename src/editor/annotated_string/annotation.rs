use super::AnnotationType;

#[derive(Copy, Clone, Debug)]
pub struct Annotation {
    pub kind: AnnotationType,
    pub start: usize,
    pub end: usize,
}
