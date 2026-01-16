use super::Highlighter;
use crate::editor::annotated_string::AnnotationType;
use crate::editor::highlight::{
    HighlightAnnotation, HighlightState, LanguageConfig, load_language_config, merge_config,
};

pub struct GenericHighlighter {
    config: LanguageConfig,
    language_name: String,
}

impl GenericHighlighter {
    pub fn new(language: &str) -> Option<Self> {
        eprintln!(
            "DEBUG: GenericHighlighter::new called for language: {}",
            language
        );
        let config = if let Ok(lang_config) = load_language_config(language, None) {
            eprintln!("DEBUG: Successfully loaded user config for {}", language);
            let default = LanguageConfig {
                keywords: vec![],
                primitive_types: vec![],
                line_comment_start: "//".to_string(),
                block_comment_start: "/*".to_string(),
                block_comment_end: "*/".to_string(),
                brackets: vec![],
            };
            merge_config(&default, Some(&lang_config))
        } else {
            eprintln!("DEBUG: User config not found, trying default config");
            // Try to load from default config in debug builds
            #[cfg(debug_assertions)]
            {
                use std::path::Path;
                let path = format!("docs/examples/default/languages/{language}.toml");
                eprintln!("DEBUG: Trying to load from: {}", path);
                if let Ok(lang_config) = load_language_config(language, Some(Path::new(&path))) {
                    eprintln!("DEBUG: Successfully loaded default config for {}", language);
                    let default = LanguageConfig {
                        keywords: vec![],
                        primitive_types: vec![],
                        line_comment_start: "//".to_string(),
                        block_comment_start: "/*".to_string(),
                        block_comment_end: "*/".to_string(),
                        brackets: vec![],
                    };
                    merge_config(&default, Some(&lang_config))
                } else {
                    eprintln!("DEBUG: Failed to load default config for {}", language);
                    return None;
                }
            }

            #[cfg(not(debug_assertions))]
            {
                return None;
            }
        };

        eprintln!("DEBUG: Successfully created {} highlighter", language);
        Some(Self {
            config,
            language_name: language.to_string(),
        })
    }
}

// The rest of the implementation is identical to RustHighlighter
// (Copy all the helper functions and impl Highlighter from rust.rs)

fn find_string_ranges(string: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut in_double_quote = false;
    let mut in_single_quote = false;
    let mut escape_next = false;
    let mut start = 0;

    for (idx, ch) in string.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        if ch == '\\' && (in_double_quote || in_single_quote) {
            escape_next = true;
            continue;
        }

        if ch == '"' && !in_single_quote {
            if in_double_quote {
                ranges.push(start..idx + 1);
                in_double_quote = false;
            } else {
                start = idx;
                in_double_quote = true;
            }
        } else if ch == '\'' && !in_double_quote {
            if in_single_quote {
                ranges.push(start..idx + 1);
                in_single_quote = false;
            } else {
                start = idx;
                in_single_quote = true;
            }
        }
    }

    ranges
}

fn is_word_boundary(ch: char) -> bool {
    !ch.is_alphanumeric() && ch != '_'
}

fn find_keyword_at(line: &str, keyword: &str, pos: usize) -> bool {
    if pos + keyword.len() > line.len() {
        return false;
    }

    let word = &line[pos..pos + keyword.len()];
    if word != keyword {
        return false;
    }

    let before_ok = pos == 0 || is_word_boundary(line.chars().nth(pos - 1).unwrap());
    let after_ok = pos + keyword.len() == line.len()
        || is_word_boundary(line.chars().nth(pos + keyword.len()).unwrap());

    before_ok && after_ok
}

impl Highlighter for GenericHighlighter {
    fn language_name(&self) -> &str {
        &self.language_name
    }

    #[allow(clippy::too_many_lines)]
    fn highlight_line(
        &self,
        line: &str,
        _line_index: usize,
        mut state: HighlightState,
    ) -> (Vec<HighlightAnnotation>, HighlightState) {
        let mut annotations = Vec::new();

        let string_ranges = find_string_ranges(line);
        for range in &string_ranges {
            annotations.push(HighlightAnnotation {
                start: range.start,
                end: range.end,
                annotation_type: AnnotationType::String,
            });
        }

        let is_in_string =
            |pos: usize| -> bool { string_ranges.iter().any(|range| range.contains(&pos)) };

        // Block comments
        let mut block_comment_ranges = Vec::new();
        if !self.config.block_comment_start.is_empty() && !self.config.block_comment_end.is_empty()
        {
            let mut pos = 0;
            while pos < line.len() {
                if state.in_block_comment {
                    if let Some(end_pos) = line[pos..].find(self.config.block_comment_end.as_str())
                    {
                        let abs_end = pos + end_pos + self.config.block_comment_end.len();
                        block_comment_ranges.push(pos..abs_end);
                        state.in_block_comment = false;
                        pos = abs_end;
                    } else {
                        block_comment_ranges.push(pos..line.len());
                        break;
                    }
                } else if let Some(start_pos) =
                    line[pos..].find(self.config.block_comment_start.as_str())
                {
                    let abs_start = pos + start_pos;
                    if is_in_string(abs_start) {
                        pos = abs_start + 1;
                    } else {
                        state.in_block_comment = true;
                        pos = abs_start + self.config.block_comment_start.len();
                    }
                } else {
                    break;
                }
            }
        }

        for range in &block_comment_ranges {
            annotations.push(HighlightAnnotation {
                start: range.start,
                end: range.end,
                annotation_type: AnnotationType::Comment,
            });
        }

        let is_in_block_comment = |pos: usize| -> bool {
            block_comment_ranges
                .iter()
                .any(|range| range.contains(&pos))
        };

        // Line comments
        if let Some(comment_start) = line.find(self.config.line_comment_start.as_str())
            && !is_in_string(comment_start)
            && !is_in_block_comment(comment_start)
        {
            annotations.push(HighlightAnnotation {
                start: comment_start,
                end: line.len(),
                annotation_type: AnnotationType::Comment,
            });
        }

        let is_in_comment = |pos: usize| -> bool {
            is_in_block_comment(pos) || {
                if let Some(comment_start) = line.find(self.config.line_comment_start.as_str()) {
                    !is_in_string(comment_start) && pos >= comment_start
                } else {
                    false
                }
            }
        };

        // Keywords
        for keyword in self.config.keywords.iter().map(std::string::String::as_str) {
            let mut search_pos = 0;
            while search_pos < line.len() {
                if let Some(found_pos) = line[search_pos..].find(keyword) {
                    let abs_pos = search_pos + found_pos;
                    if find_keyword_at(line, keyword, abs_pos)
                        && !is_in_string(abs_pos)
                        && !is_in_comment(abs_pos)
                    {
                        annotations.push(HighlightAnnotation {
                            start: abs_pos,
                            end: abs_pos + keyword.len(),
                            annotation_type: AnnotationType::Keyword,
                        });
                    }
                    search_pos = abs_pos + 1;
                } else {
                    break;
                }
            }
        }

        // Primitive types
        for prim_type in self
            .config
            .primitive_types
            .iter()
            .map(std::string::String::as_str)
        {
            let mut search_pos = 0;
            while search_pos < line.len() {
                if let Some(found_pos) = line[search_pos..].find(prim_type) {
                    let abs_pos = search_pos + found_pos;
                    if find_keyword_at(line, prim_type, abs_pos)
                        && !is_in_string(abs_pos)
                        && !is_in_comment(abs_pos)
                    {
                        annotations.push(HighlightAnnotation {
                            start: abs_pos,
                            end: abs_pos + prim_type.len(),
                            annotation_type: AnnotationType::PrimitiveType,
                        });
                    }
                    search_pos = abs_pos + 1;
                } else {
                    break;
                }
            }
        }

        // Numbers
        let mut chars = line.chars().enumerate().peekable();
        while let Some((idx, ch)) = chars.next() {
            if is_in_string(idx) || is_in_comment(idx) {
                continue;
            }

            if ch.is_ascii_digit()
                || (ch == '0'
                    && chars.peek().is_some_and(|(_, next_ch)| {
                        *next_ch == 'x' || *next_ch == 'b' || *next_ch == 'o'
                    }))
            {
                let start = idx;
                let mut end = idx + 1;

                if ch == '0'
                    && let Some((_, next_ch)) = chars.peek()
                    && (*next_ch == 'x' || *next_ch == 'b' || *next_ch == 'o')
                {
                    chars.next();
                    end += 1;
                }

                while let Some(&(_, next_ch)) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '.' {
                        chars.next();
                        end += 1;
                    } else {
                        break;
                    }
                }

                annotations.push(HighlightAnnotation {
                    start,
                    end,
                    annotation_type: AnnotationType::Number,
                });
            }
        }

        // Type names (capitalized identifiers)
        let mut chars = line.chars().enumerate().peekable();
        while let Some((idx, ch)) = chars.next() {
            if is_in_string(idx) || is_in_comment(idx) {
                continue;
            }

            if ch.is_uppercase() {
                let start = idx;
                let mut end = idx + 1;

                while let Some(&(_, next_ch)) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        chars.next();
                        end += 1;
                    } else {
                        break;
                    }
                }

                let word = &line[start..end];
                if !self.config.keywords.iter().any(|kw| kw == word)
                    && !self.config.primitive_types.iter().any(|pt| pt == word)
                {
                    annotations.push(HighlightAnnotation {
                        start,
                        end,
                        annotation_type: AnnotationType::Type,
                    });
                }
            }
        }

        // Brackets
        let mut paren_level = state.paren_level;
        let mut brace_level = state.brace_level;
        let mut bracket_level = state.bracket_level;

        for (idx, ch) in line.chars().enumerate() {
            if is_in_string(idx) || is_in_comment(idx) {
                continue;
            }

            if ch == '(' {
                let color_index = paren_level % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
                paren_level += 1;
            } else if ch == ')' {
                paren_level = paren_level.saturating_sub(1);
                let color_index = paren_level % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
            } else if ch == '{' {
                let color_index = (brace_level + 1) % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
                brace_level += 1;
            } else if ch == '}' {
                brace_level = brace_level.saturating_sub(1);
                let color_index = (brace_level + 1) % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
            } else if ch == '[' {
                let color_index = (bracket_level + 2) % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
                bracket_level += 1;
            } else if ch == ']' {
                bracket_level = bracket_level.saturating_sub(1);
                let color_index = (bracket_level + 2) % 4;
                let annotation_type = match color_index {
                    0 => AnnotationType::Bracket0,
                    1 => AnnotationType::Bracket1,
                    2 => AnnotationType::Bracket2,
                    _ => AnnotationType::Bracket3,
                };
                annotations.push(HighlightAnnotation {
                    start: idx,
                    end: idx + 1,
                    annotation_type,
                });
            }
        }

        state.paren_level = paren_level;
        state.brace_level = brace_level;
        state.bracket_level = bracket_level;

        (annotations, state)
    }
}
