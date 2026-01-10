use std::{
    cmp::{max, min},
    fmt::{self, Display},
};
pub mod annotation_type;
pub use annotation_type::AnnotationType;
mod annotation;
use annotation::Annotation;
mod annotated_string_part;
use annotated_string_part::AnnotatedStringPart;
mod annotated_string_iterator;
use annotated_string_iterator::AnnotatedStringIterator;

#[derive(Default, Debug)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }
    pub fn add_annotation(&mut self, annotation_type: AnnotationType, start: usize, end: usize) {
        debug_assert!(start <= end);
        self.annotations.push(Annotation {
            kind: annotation_type,
            start: start,
            end,
        });
    }

    pub fn replace(&mut self, start: usize, end: usize, new_string: &str) {
        debug_assert!(start <= end);

        let end = min(end, self.string.len());
        if start > end {
            return;
        }
        self.string.replace_range(start..end, new_string);

        let replaced_range_len = end.saturating_sub(start);
        let shortened = new_string.len() < replaced_range_len;
        let len_difference = new_string.len().abs_diff(replaced_range_len);

        if len_difference == 0 {
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            annotation.start = if annotation.start >= end {
                if shortened {
                    annotation.start.saturating_sub(len_difference)
                } else {
                    annotation.start.saturating_add(len_difference)
                }
            } else if annotation.start >= start {
                if shortened {
                    max(start, annotation.start.saturating_sub(len_difference))
                } else {
                    min(end, annotation.start.saturating_add(len_difference))
                }
            } else {
                annotation.start
            };

            annotation.end = if annotation.end >= end {
                if shortened {
                    annotation.end.saturating_sub(len_difference)
                } else {
                    annotation.end.saturating_add(len_difference)
                }
            } else if annotation.end >= start {
                if shortened {
                    max(start, annotation.end.saturating_sub(len_difference))
                } else {
                    min(end, annotation.end.saturating_add(len_difference))
                }
            } else {
                annotation.end
            }
        });

        self.annotations.retain(|annotation| {
            annotation.start < annotation.end && annotation.start < self.string.len()
        });
    }
}

impl Display for AnnotatedString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

impl<'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_idx: 0,
        }
    }
}
