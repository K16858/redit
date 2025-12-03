use super::{
    editor_command::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
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
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size(),
            location: Location::default(),
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
        let Size { width, .. } = Terminal::size();
        for (i, line) in self.buffer.lines.iter().enumerate() {
            Terminal::move_caret_to(Position { col: 0, row: i })?;
            Terminal::clear_line()?;
            Terminal::print(&line.get(0..width))?;
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
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
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
        self.location = Location { x, y };
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
