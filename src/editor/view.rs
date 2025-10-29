use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer_screen()?;
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn render_welcome_screen() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        let vertical_center = height / 3;

        for current_row in 0..height {
            if current_row == vertical_center {
                Self::draw_welcome_message(current_row)?;
            } else {
                Self::draw_empty_row(current_row)?;
            }
        }
        Ok(())
    }

    fn render_buffer_screen(&self) -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;
        for (i, line) in self.buffer.lines.iter().enumerate() {
            let truncated_line = if line.len() >= width {
                &line[0..width]
            } else {
                line
            };
            Terminal::move_caret_to(Position { col: 0, row: i })?;
            Terminal::clear_line()?;
            Terminal::print(truncated_line)?;
        }
        Ok(())
    }

    fn draw_welcome_message(at: usize) -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Self::render_line(at, &welcome_message)?;
        Ok(())
    }

    fn draw_empty_row(at: usize) -> Result<(), Error> {
        Self::render_line(at, "~")?;
        Ok(())
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}
