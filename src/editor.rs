use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
