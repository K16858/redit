use self::line::Line;
use super::{
    editor_command::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
use std::cmp::min;
mod buffer;
mod line;
mod location;
use buffer::Buffer;
use location::Location;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
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
        let Size { height, .. } = Terminal::size();
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
        let Size { width, height } = Terminal::size();
        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            }
        }
        Ok(())
    }

    fn draw_welcome_message(at: usize) -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size().width;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Self::render_line(at, &welcome_message);
        Ok(())
    }

    fn draw_empty_row(at: usize) -> Result<(), Error> {
        Self::render_line(at, "~");
        Ok(())
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::len);
                }
            }
            Direction::Right => {
                let width = self.buffer.lines.get(y).map_or(0, Line::len);
                if x < width {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = width.saturating_sub(1);
            }
        }
        x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| min(line.len(), x));
        y = min(y, self.buffer.lines.len());

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}
