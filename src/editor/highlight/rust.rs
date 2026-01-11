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

fn find_number_ranges(string: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut in_number = false;
    let mut number_start = 0;
    let mut has_dot = false;
    let mut chars = string.char_indices().peekable();

    while let Some((byte_idx, ch)) = chars.next() {
        if !in_number {
            if ch.is_ascii_digit() {
                in_number = true;
                number_start = byte_idx;
                has_dot = false;
            }
        } else {
            if ch.is_ascii_digit() {
            } else if ch == '.' && !has_dot {
                has_dot = true;
            } else {
                in_number = false;
                ranges.push(number_start..byte_idx);
            }
        }
    }

    if in_number {
        ranges.push(number_start..string.len());
    }

    ranges
}

fn find_type_ranges(string: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut in_type = false;
    let mut type_start = 0;

    for (byte_idx, ch) in string.char_indices() {
        if !in_type {
            if ch.is_uppercase() {
                in_type = true;
                type_start = byte_idx;
            }
        } else {
            if ch.is_alphanumeric() || ch == '_' {
            } else {
                in_type = false;
                ranges.push(type_start..byte_idx);
            }
        }
    }

    if in_type {
        ranges.push(type_start..string.len());
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

        let mut comment_ranges = Vec::new();
        if let Some(comment_start) = line.find("//") {
            if !is_in_string(comment_start) {
                comment_ranges.push(comment_start..line.len());
            }
        }
        let mut search_start = 0;
        while let Some(rel_pos) = line[search_start..].find("/*") {
            let open_byte = search_start + rel_pos;
            if !is_in_string(open_byte) {
                if let Some(rel_close_pos) = line[open_byte + 2..].find("*/") {
                    let close_byte = open_byte + 2 + rel_close_pos + 2;
                    comment_ranges.push(open_byte..close_byte);
                    search_start = close_byte;
                } else {
                    comment_ranges.push(open_byte..line.len());
                    break;
                }
            } else {
                search_start = open_byte + 2;
            }
        }
        let is_in_comment = |byte_idx: usize| -> bool {
            comment_ranges.iter().any(|range| range.contains(&byte_idx))
        };

        let number_ranges = find_number_ranges(line);
        for range in &number_ranges {
            if !is_in_string(range.start) && !is_in_comment(range.start) {
                annotations.push(HighlightAnnotation {
                    annotation_type: AnnotationType::Number,
                    start: range.start,
                    end: range.end,
                });
            }
        }

        let type_ranges = find_type_ranges(line);
        for range in &type_ranges {
            if !is_in_string(range.start) && !is_in_comment(range.start) {
                let word = &line[range.start..range.end];
                let is_keyword = keywords.iter().any(|&kw| kw == word);
                if !is_keyword {
                    annotations.push(HighlightAnnotation {
                        annotation_type: AnnotationType::Type,
                        start: range.start,
                        end: range.end,
                    });
                }
            }
        }

        for range in &comment_ranges {
            annotations.push(HighlightAnnotation {
                annotation_type: AnnotationType::Comment,
                start: range.start,
                end: range.end,
            });
        }

        annotations
    }

    fn language_name(&self) -> &str {
        "rust"
    }
}
