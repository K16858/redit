use super::{AnnotatedString, AnnotatedStringPart, AnnotationType};
use std::cmp::min;

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.annotated_string.string.len() {
            return None;
        }

        let covering: Vec<_> = self
            .annotated_string
            .annotations
            .iter()
            .filter(|a| a.start <= self.current_idx && a.end > self.current_idx)
            .collect();

        let annotation = covering
            .iter()
            .find(|a| a.kind == AnnotationType::Selection)
            .map(|a| *a)
            .or_else(|| covering.last().map(|v| *v));
        if let Some(annotation) = annotation {
            let start_idx = self.current_idx;
            let mut end_idx = min(annotation.end, self.annotated_string.string.len());
            // Don't skip over the start of another annotation: split at boundaries
            // so that e.g. Selection (2..5) inside Comment (0..74) gets emitted.
            for a in &self.annotated_string.annotations {
                if a.start > start_idx && a.start < end_idx {
                    end_idx = a.start;
                }
            }
            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: Some(annotation.kind),
            });
        }
        let mut end_idx = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start > self.current_idx && annotation.start < end_idx {
                end_idx = annotation.start;
            }
        }
        let start_idx = self.current_idx;
        self.current_idx = end_idx;

        Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotation_type: None,
        })
    }
}
