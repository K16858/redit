use crossterm::event::{
    KeyCode::{Down, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]
pub struct Move {
    pub direction: MoveDirection,
    pub is_selection: bool,
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    PageUp,
    PageDown,
    ScrollUp,
    ScrollDown,
    LineStart,
    LineEnd,
    Up,
    Left,
    Right,
    Down,
}

impl Move {
    pub fn left(is_selection: bool) -> Self {
        Self {
            direction: MoveDirection::Left,
            is_selection,
        }
    }

    pub fn right(is_selection: bool) -> Self {
        Self {
            direction: MoveDirection::Right,
            is_selection,
        }
    }
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        let is_selection = modifiers.contains(KeyModifiers::SHIFT);
        let direction = match (code, modifiers) {
            (Up, KeyModifiers::CONTROL) => MoveDirection::ScrollUp,
            (Down, KeyModifiers::CONTROL) => MoveDirection::ScrollDown,
            (PageUp, _) => MoveDirection::PageUp,
            (PageDown, _) => MoveDirection::PageDown,
            (Left, KeyModifiers::CONTROL) => MoveDirection::LineStart,
            (Right, KeyModifiers::CONTROL) => MoveDirection::LineEnd,
            (Up, _) => MoveDirection::Up,
            (Down, _) => MoveDirection::Down,
            (Left, _) => MoveDirection::Left,
            (Right, _) => MoveDirection::Right,
            _ => return Err(format!("Key Code not supported: {code:?}")),
        };
        Ok(Self {
            direction,
            is_selection,
        })
    }
}
