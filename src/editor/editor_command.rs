use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

pub enum MoveCommand {
    PageUp,
    PageDown,
    LineStart,
    LineEnd,
    Up,
    Left,
    Right,
    Down,
}
pub enum EditorCommand {
    Move(MoveCommand),
    Resize(Size),
    Quit,
    Insert(char),
    Backspace,
    Delete,
    Enter,
    Save,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                }
                (KeyCode::Up, KeyModifiers::CONTROL) => Ok(Self::Move(MoveCommand::PageUp)),
                (KeyCode::Down, KeyModifiers::CONTROL) => Ok(Self::Move(MoveCommand::PageDown)),
                (KeyCode::Left, KeyModifiers::CONTROL) => Ok(Self::Move(MoveCommand::LineStart)),
                (KeyCode::Right, KeyModifiers::CONTROL) => Ok(Self::Move(MoveCommand::LineEnd)),
                (KeyCode::Up, _) => Ok(Self::Move(MoveCommand::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(MoveCommand::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(MoveCommand::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(MoveCommand::Right)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
