mod terminal;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
mod document_status;
use document_status::DocumentStatus;
mod fileinfo;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
mod view;
use view::View;
mod editor_command;
use editor_command::EditorCommand;
mod message_bar;
mod status_bar;
use ui_component::UIComponent;
mod ui_component;
use self::{message_bar::MessageBar, terminal::Size};
use status_bar::StatusBar;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    terminal_size: Size,
    message_bar: MessageBar,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }

        editor
            .message_bar
            .update_message("HELP: Ctrl-S = save | Ctrl-Q = quit".to_string());

        editor.refresh_status();
        Ok(editor)
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        self.repl();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else if let EditorCommand::Resize(size) = command {
                    self.resize(size);
                } else {
                    self.view.handle_command(command);
                }
            }
        } else {
            return;
        }
    }

    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_caret();
        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            let _ = self.view.draw(0);
        }

        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn repl(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
    }
}
