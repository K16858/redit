use std::{
    cmp::min,
    fmt,
    ops::{Deref, Range},
};

mod grapheme_width;
mod text_fragment;
use grapheme_width::GraphemeWidth;
use text_fragment::TextFragment;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::highlight::{HighlightAnnotation, Highlighter};
use super::{AnnotatedString, AnnotationType};
use crate::editor::highlight::HighlightState;

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
            string: String::from(line_str),
        }
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .grapheme_indices(true)
            .map(|(byte_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start: byte_idx,
                }
            })
            .collect()
    }

    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next()
                    && ch.is_control()
                    && chars.next().is_none()
                {
                    return Some('▯');
                }
                Some('·')
            }
            _ => None,
        }
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        self.get_annotated_visible_substr(range, None, None, None, HighlightState::default(), None)
            .0
            .to_string()
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn get_annotated_visible_substr(
        &self,
        range: Range<usize>,
        query: Option<&str>,
        selected_match: Option<usize>,
        highlighter: Option<&dyn Highlighter>,
        state: HighlightState,
        cached_annotations: Option<&[HighlightAnnotation]>,
    ) -> (AnnotatedString, HighlightState) {
        if range.start >= range.end {
            return (AnnotatedString::default(), state);
        }

        let mut result = AnnotatedString::from(&self.string);
        let mut new_state = state;
        if let Some(cached) = cached_annotations {
            for highlight in cached {
                result.add_annotation(highlight.annotation_type, highlight.start, highlight.end);
            }
        } else if let Some(hl) = highlighter {
            let (highlights, updated_state) = hl.highlight_line(&self.string, 0, state);
            new_state = updated_state;
            for highlight in highlights {
                result.add_annotation(highlight.annotation_type, highlight.start, highlight.end);
            }
        }

        if let Some(query) = query
            && !query.is_empty()
        {
            self.find_all(query, 0..self.string.len())
                .iter()
                .for_each(|(start, grapheme_idx)| {
                    if let Some(selected_match) = selected_match
                        && *grapheme_idx == selected_match
                    {
                        result.add_annotation(
                            AnnotationType::SelectedMatch,
                            *start,
                            start.saturating_add(query.len()),
                        );
                        return;
                    }
                    result.add_annotation(
                        AnnotationType::Match,
                        *start,
                        start.saturating_add(query.len()),
                    );
                });
        }

        let mut fragment_start = self.width();
        for fragment in self.fragments.iter().rev() {
            let fragment_end = fragment_start;
            fragment_start = fragment_start.saturating_sub(fragment.rendered_width.into());

            if fragment_start > range.end {
                continue;
            }

            if fragment_start < range.end && fragment_end > range.end {
                result.replace(fragment.start, self.string.len(), "⋯");
                continue;
            } else if fragment_start == range.end {
                result.truncate_right_from(fragment.start);
                continue;
            }

            if fragment_end <= range.start {
                result.truncate_left_until(fragment.start.saturating_add(fragment.grapheme.len()));
                break;
            } else if fragment_start < range.start && fragment_end > range.start {
                result.replace(
                    0,
                    fragment.start.saturating_add(fragment.grapheme.len()),
                    "⋯",
                );
                break;
            }

            if fragment_start >= range.start
                && fragment_end <= range.end
                && let Some(replacement) = fragment.replacement
            {
                let start = fragment.start;
                let end = start.saturating_add(fragment.grapheme.len());
                result.replace(start, end, &replacement.to_string());
            }
        }
        (result, new_state)
    }

    pub fn width_until(&self, grapheme_idx: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_idx)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn insert_char(&mut self, character: char, at: usize) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start, character);
        } else {
            self.string.push(character);
        }

        self.rebuild_fragments();
    }

    pub fn width(&self) -> usize {
        self.width_until(self.grapheme_count())
    }

    pub fn append_char(&mut self, character: char) {
        self.insert_char(character, self.grapheme_count());
    }

    pub fn delete_last(&mut self) {
        self.delete(self.grapheme_count().saturating_sub(1));
    }

    pub fn delete(&mut self, at: usize) {
        if let Some(fragment) = self.fragments.get(at) {
            let start = fragment.start;
            let end = fragment.start.saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
        }
    }

    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.rebuild_fragments();
    }

    pub fn split(&mut self, at: usize) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start);
            self.rebuild_fragments();
            Self::from(&remainder)
        } else {
            Self::default()
        }
    }

    fn byte_idx_to_grapheme_idx(&self, byte_idx: usize) -> Option<usize> {
        if byte_idx > self.string.len() {
            return None;
        }
        self.fragments
            .iter()
            .position(|fragment| fragment.start >= byte_idx)
    }

    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: usize) -> usize {
        self.fragments
            .get(grapheme_idx)
            .map_or(0, |fragment| fragment.start)
    }

    pub fn grapheme_to_byte_idx(&self, grapheme_idx: usize) -> usize {
        self.grapheme_idx_to_byte_idx(grapheme_idx)
    }

    pub fn line_length(&self) -> usize {
        self.string.len()
    }

    pub fn search_forward(&self, query: &str, from_grapheme_idx: usize) -> Option<usize> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == self.grapheme_count() {
            return None;
        }
        let start = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
        self.find_all(query, start..self.string.len())
            .first()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    pub fn search_backward(&self, query: &str, from_grapheme_idx: usize) -> Option<usize> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());

        if from_grapheme_idx == 0 {
            return None;
        }
        let end_byte_index = if from_grapheme_idx == self.grapheme_count() {
            self.string.len()
        } else {
            self.grapheme_idx_to_byte_idx(from_grapheme_idx)
        };
        self.find_all(query, 0..end_byte_index)
            .last()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    fn find_all(&self, query: &str, range: Range<usize>) -> Vec<(usize, usize)> {
        let end = min(range.end, self.string.len());
        let start = range.start;

        debug_assert!(start <= end);
        debug_assert!(start <= self.string.len());

        self.string.get(start..end).map_or_else(Vec::new, |substr| {
            let potential_matches: Vec<usize> = substr
                .match_indices(query)
                .map(|(relative_start_idx, _)| relative_start_idx.saturating_add(start))
                .collect();
            self.match_graphme_clusters(&potential_matches, query)
        })
    }

    fn match_graphme_clusters(&self, matches: &[usize], query: &str) -> Vec<(usize, usize)> {
        let grapheme_count = query.graphemes(true).count();
        matches
            .iter()
            .filter_map(|&start| {
                self.byte_idx_to_grapheme_idx(start)
                    .and_then(|grapheme_idx| {
                        self.fragments
                            .get(grapheme_idx..grapheme_idx.saturating_add(grapheme_count))
                            .and_then(|fragments| {
                                let substring = fragments
                                    .iter()
                                    .map(|fragment| fragment.grapheme.as_str())
                                    .collect::<String>();
                                (substring == query).then_some((start, grapheme_idx))
                            })
                    })
            })
            .collect()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
