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
    let mut i = 0;
    let chars: Vec<_> = string.char_indices().collect();

    while i < chars.len() {
        let (byte_idx, ch) = chars[i];

        if ch == '0' && i + 1 < chars.len() {
            let next_ch = chars[i + 1].1;
            if next_ch == 'x' || next_ch == 'X' {
                let start = byte_idx;
                let mut j = i + 2;
                while j < chars.len() {
                    let digit_ch = chars[j].1;
                    if digit_ch.is_ascii_hexdigit() {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j > i + 2 {
                    ranges.push(start..chars[j].0);
                }
                i = j;
                continue;
            } else if next_ch == 'o' || next_ch == 'O' {
                let start = byte_idx;
                let mut j = i + 2;
                while j < chars.len() {
                    let digit_ch = chars[j].1;
                    if digit_ch >= '0' && digit_ch <= '7' {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j > i + 2 {
                    ranges.push(start..chars[j].0);
                }
                i = j;
                continue;
            } else if next_ch == 'b' || next_ch == 'B' {
                let start = byte_idx;
                let mut j = i + 2;
                while j < chars.len() {
                    let digit_ch = chars[j].1;
                    if digit_ch == '0' || digit_ch == '1' {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j > i + 2 {
                    ranges.push(start..chars[j].0);
                }
                i = j;
                continue;
            }
        }

        if ch.is_ascii_digit() {
            let start = byte_idx;
            let mut j = i;
            let mut has_dot = false;

            while j < chars.len() {
                let digit_ch = chars[j].1;
                if digit_ch.is_ascii_digit() {
                    j += 1;
                } else if digit_ch == '.'
                    && !has_dot
                    && j + 1 < chars.len()
                    && chars[j + 1].1.is_ascii_digit()
                {
                    has_dot = true;
                    j += 1;
                } else {
                    break;
                }
            }
            ranges.push(start..chars[j].0);
            i = j;
        } else {
            i += 1;
        }
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
    fn highlight_line(
        &self,
        line: &str,
        _line_idx: usize,
        in_block_comment: bool,
    ) -> (Vec<HighlightAnnotation>, bool) {
        let mut annotations = Vec::new();
        let mut still_in_block_comment = in_block_comment;

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
        if still_in_block_comment {
            if let Some(close_pos) = line.find("*/") {
                let close_byte = close_pos + 2;
                comment_ranges.push(0..close_byte);
                still_in_block_comment = false;
            } else {
                comment_ranges.push(0..line.len());
            }
        } else {
            if let Some(comment_start) = line.find("//") {
                if !is_in_string(comment_start) {
                    comment_ranges.push(comment_start..line.len());
                }
            }
        }
        let mut search_start = if still_in_block_comment {
            line.len()
        } else {
            0
        };
        while let Some(rel_pos) = line[search_start..].find("/*") {
            let open_byte = search_start + rel_pos;
            if !is_in_string(open_byte) {
                if let Some(rel_close_pos) = line[open_byte + 2..].find("*/") {
                    let close_byte = open_byte + 2 + rel_close_pos + 2;
                    comment_ranges.push(open_byte..close_byte);
                    search_start = close_byte;
                } else {
                    comment_ranges.push(open_byte..line.len());
                    still_in_block_comment = true;
                    break;
                }
            } else {
                search_start = open_byte + 2;
            }
        }
        let is_in_comment = |byte_idx: usize| -> bool {
            comment_ranges.iter().any(|range| range.contains(&byte_idx))
        };

        let primitive_types = [
            "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
        ];
        for primitive_type in primitive_types {
            let mut search_start = 0;
            while let Some(rel_pos) = line[search_start..].find(primitive_type) {
                let start = search_start + rel_pos;
                let end = start + primitive_type.len();

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
                    if !is_in_string(start) && !is_in_comment(start) {
                        annotations.push(HighlightAnnotation {
                            annotation_type: AnnotationType::PrimitiveType,
                            start,
                            end,
                        });
                    }
                }
                search_start = start + 1;
            }
        }

        let number_ranges = find_number_ranges(line);
        for range in &number_ranges {
            if !is_in_string(range.start) && !is_in_comment(range.start) {
                let is_word_boundary_before = range.start == 0
                    || !line[..range.start]
                        .chars()
                        .last()
                        .map_or(false, |c| c.is_alphanumeric() || c == '_');
                let is_word_boundary_after = range.end >= line.len()
                    || !line[range.end..]
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_alphanumeric() || c == '_');

                if is_word_boundary_before && is_word_boundary_after {
                    annotations.push(HighlightAnnotation {
                        annotation_type: AnnotationType::Number,
                        start: range.start,
                        end: range.end,
                    });
                }
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

        let mut paren_level: usize = 0;
        let mut brace_level: usize = 0;
        let mut bracket_level: usize = 0;

        for (byte_idx, ch) in line.char_indices() {
            if !is_in_string(byte_idx) && !is_in_comment(byte_idx) {
                let bracket_type = match ch {
                    '(' => {
                        let level = (paren_level + 0) % 4;
                        paren_level += 1;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    ')' => {
                        paren_level = paren_level.saturating_sub(1);
                        let level = (paren_level + 0) % 4;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    '{' => {
                        let level = (brace_level + 1) % 4;
                        brace_level += 1;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    '}' => {
                        brace_level = brace_level.saturating_sub(1);
                        let level = (brace_level + 1) % 4;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    '[' => {
                        let level = (bracket_level + 2) % 4;
                        bracket_level += 1;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    ']' => {
                        bracket_level = bracket_level.saturating_sub(1);
                        let level = (bracket_level + 2) % 4;
                        Some(match level {
                            0 => AnnotationType::Bracket0,
                            1 => AnnotationType::Bracket1,
                            2 => AnnotationType::Bracket2,
                            3 => AnnotationType::Bracket3,
                            _ => unreachable!(),
                        })
                    }
                    _ => None,
                };

                if let Some(annotation_type) = bracket_type {
                    annotations.push(HighlightAnnotation {
                        annotation_type,
                        start: byte_idx,
                        end: byte_idx + ch.len_utf8(),
                    });
                }
            }
        }

        (annotations, still_in_block_comment)
    }

    fn language_name(&self) -> &str {
        "rust"
    }
}
