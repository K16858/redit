use super::AnnotationType;

#[derive(Copy, Clone, Debug)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start_byte_idx: usize,
    pub end_byte_idx: usize,
}
