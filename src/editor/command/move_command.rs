use crossterm::event::{
    KeyCode::{Down, Left, Right, Up},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    LineStart,
    LineEnd,

    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        match (code, modifiers) {
            (Up, KeyModifiers::CONTROL) => Ok(Self::PageUp),
            (Down, KeyModifiers::CONTROL) => Ok(Self::PageDown),
            (Left, KeyModifiers::CONTROL) => Ok(Self::LineStart),
            (Right, KeyModifiers::CONTROL) => Ok(Self::LineEnd),
            (Up, _) => Ok(Self::Up),
            (Down, _) => Ok(Self::Down),
            (Left, _) => Ok(Self::Left),
            (Right, _) => Ok(Self::Right),
            _ => Err(format!("Key Code not supported: {code:?}")),
        }
    }
}
