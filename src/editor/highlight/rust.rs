use super::{HighlightAnnotation, Highlighter};
use crate::editor::annotated_string::AnnotationType;

pub struct RustHighlighter;

fn find_string_ranges(string: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut in_string = false;
    let mut string_start = 0;
    let mut chars = string.char_indices().peekable();

    while let Some((byte_idx, ch)) = chars.next() {
        if !in_string {
            if ch == '"' {
                in_string = true;
                string_start = byte_idx;
            }
        } else {
            if ch == '\\' {
                chars.next();
                continue;
            }
            if ch == '"' {
                in_string = false;
                ranges.push(string_start..byte_idx + ch.len_utf8());
            }
        }
    }
    ranges
}

impl Highlighter for RustHighlighter {
    fn highlight_line(&self, line: &str, _line_idx: usize) -> Vec<HighlightAnnotation> {
        let mut annotations = Vec::new();

        let keywords = ["fn", "let", "mut", "if", "else", "for", "while", "match"];
        for keyword in keywords {
            let mut search_start = 0;
            while let Some(rel_pos) = line[search_start..].find(keyword) {
                let start = search_start + rel_pos;
                let end = start + keyword.len();

                let is_word_boundary_before = start == 0
                    || !line[..start]
                        .chars()
                        .last()
                        .map_or(false, |c| c.is_alphanumeric() || c == '_');
                let is_word_boundary_after = end >= line.len()
                    || !line[end..]
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_alphanumeric() || c == '_');

                if is_word_boundary_before && is_word_boundary_after {
                    annotations.push(HighlightAnnotation {
                        annotation_type: AnnotationType::Keyword,
                        start,
                        end,
                    });
                }
                search_start = start + 1;
            }
        }
        let string_ranges = find_string_ranges(line);

        for range in &string_ranges {
            annotations.push(HighlightAnnotation {
                annotation_type: AnnotationType::String,
                start: range.start,
                end: range.end,
            });
        }

        let is_in_string = |byte_idx: usize| -> bool {
            string_ranges.iter().any(|range| range.contains(&byte_idx))
        };

        if let Some(comment_start) = line.find("//") {
            if !is_in_string(comment_start) {
                let comment_end = line.len();
                annotations.push(HighlightAnnotation {
                    annotation_type: AnnotationType::Comment,
                    start: comment_start,
                    end: comment_end,
                });
            }
        }

        let mut search_start = 0;
        while let Some(rel_pos) = line[search_start..].find("/*") {
            let open_byte = search_start + rel_pos;
            if !is_in_string(open_byte) {
                if let Some(rel_close_pos) = line[open_byte + 2..].find("*/") {
                    let close_byte = open_byte + 2 + rel_close_pos + 2;
                    annotations.push(HighlightAnnotation {
                        annotation_type: AnnotationType::Comment,
                        start: open_byte,
                        end: close_byte,
                    });
                    search_start = close_byte;
                } else {
                    annotations.push(HighlightAnnotation {
                        annotation_type: AnnotationType::Comment,
                        start: open_byte,
                        end: line.len(),
                    });
                    break;
                }
            } else {
                search_start = open_byte + 2;
            }
        }

        annotations
    }

    fn language_name(&self) -> &str {
        "rust"
    }
}
