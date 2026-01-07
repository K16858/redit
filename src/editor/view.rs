use self::line::Line;
use super::{
    DocumentStatus, NAME, VERSION,
    editor_command::{EditorCommand, MoveCommand},
    terminal::{Position, Size, Terminal},
    ui_component::UIComponent,
};
use std::cmp::min;
mod buffer;
mod line;
use buffer::Buffer;
use std::io::Error;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            file_name: format!("{}", self.buffer.file_info),
            is_modified: self.buffer.modified,
        }
    }

    fn render_welcome_screen(&self, origin_y: usize) -> Result<(), Error> {
        let Size { height, .. } = self.size;
        let vertical_center = height / 3;

        for row in 0..height {
            let draw_row = origin_y + row;
            if row == vertical_center {
                Self::draw_welcome_message(draw_row)?;
            } else {
                Self::draw_empty_row(draw_row)?;
            }
        }
        Ok(())
    }

    fn render_buffer(&self, origin_y: usize) -> Result<(), Error> {
        let Size { height, width } = self.size;
        let top = self.scroll_offset.row;

        for screen_row in 0..height {
            let line_idx = top + screen_row;
            let draw_row = origin_y + screen_row;

            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = left + width;
                Self::render_line(draw_row, &line.get_visible_graphemes(left..right))?;
            } else {
                Self::render_line(draw_row, "~")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message(at: usize) -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size().unwrap().width;
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
        Terminal::print_row(at, line_text)
    }

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    fn move_text_location(&mut self, direction: MoveCommand) {
        let Size { height, .. } = self.size;
        match direction {
            MoveCommand::Up => self.move_up(1),
            MoveCommand::Down => self.move_down(1),
            MoveCommand::Left => self.move_left(),
            MoveCommand::Right => self.move_right(),
            MoveCommand::PageUp => self.move_up(height.saturating_sub(1)),
            MoveCommand::PageDown => self.move_down(height.saturating_sub(1)),
            MoveCommand::LineStart => self.move_to_start_of_line(),
            MoveCommand::LineEnd => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }
    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(true);
        }
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(true);
        }
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(direction),
            EditorCommand::Quit => {}
            EditorCommand::Insert(character) => self.insert_char(character),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Enter => self.insert_newline(),
            EditorCommand::Save => self.save(),
        }
    }

    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        self.buffer.insert_char(character, self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            self.move_text_location(MoveCommand::Right);
        }
        self.mark_redraw(true);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(MoveCommand::Right);
        self.mark_redraw(true);
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.mark_redraw(true);
        }
    }

    fn backspace(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.move_text_location(MoveCommand::Left);
            self.delete();
        }
    }

    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.mark_redraw(true);
    }

    fn save(&mut self) {
        let _ = self.buffer.save();
    }
}

impl UIComponent for View {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        if self.buffer.is_empty() {
            self.render_welcome_screen(origin_y)
        } else {
            self.render_buffer(origin_y)
        }
    }
}
