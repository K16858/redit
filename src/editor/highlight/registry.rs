use super::Highlighter;
use crate::editor::highlight::rust::RustHighlighter;
use std::collections::HashMap;

pub struct HighlighterRegistry {
    highlighters: Vec<Box<dyn Highlighter>>,
    extension_map: HashMap<String, usize>,
}

impl HighlighterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            highlighters: Vec::new(),
            extension_map: HashMap::new(),
        };

        registry.register(Box::new(RustHighlighter), vec!["rs".to_string()]);

        registry
    }

    pub fn register(&mut self, highlighter: Box<dyn Highlighter>, extensions: Vec<String>) {
        let index = self.highlighters.len();
        self.highlighters.push(highlighter);

        for ext in extensions {
            self.extension_map.insert(ext, index);
        }
    }

    pub fn get_highlighter(&self, file_extension: Option<&str>) -> Option<&dyn Highlighter> {
        let ext = file_extension?;
        let index = self.extension_map.get(ext)?;
        self.highlighters.get(*index).map(|h| h.as_ref())
    }
}

impl Default for HighlighterRegistry {
    fn default() -> Self {
        Self::new()
    }
}
