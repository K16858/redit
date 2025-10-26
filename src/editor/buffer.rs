pub struct BUffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec![String::from("Hello, World!")],
        }
    }
}
