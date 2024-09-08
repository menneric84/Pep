use std::io::{stdout, Stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{read, Event, KeyCode, KeyEvent},
    style,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use serde::{Deserialize, Serialize};

use crate::window::Window;
use crate::config::{Config, KeyAction};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    out: Stdout,
    config: Config,
    current_buffer: Window,
    alt_buffers: Vec<Window>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    InsertMode,
    DeleteUnderCursor,
    NormalMode,
}

impl Editor {
    fn move_cursor(&mut self, x: u16, y: u16) {
        self.current_buffer.cursor.x = x;
        self.current_buffer.cursor.y = y;
        self.out.queue(cursor::MoveTo(x, y)).unwrap();
    }

    fn enter_insert_mode(&mut self) {
        self.current_buffer.mode = Mode::Insert;
        self.out.queue(SetCursorStyle::BlinkingBar).unwrap();
    }

    fn enter_normal_mode(&mut self) {
        self.current_buffer.mode = Mode::Normal;
        self.out.queue(SetCursorStyle::DefaultUserShape).unwrap();
    }

    fn clear(&mut self) {
        self.out.execute(terminal::Clear(ClearType::All)).unwrap();
    }

    fn enter_alt_screen(&mut self) {
        self.out.execute(EnterAlternateScreen).unwrap();
    }

    fn leave_alt_screen(&mut self) {
        self.out.execute(LeaveAlternateScreen).unwrap();
    }

    fn raw(&mut self) {
        terminal::enable_raw_mode().unwrap();
    }

    fn disable_raw(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }

    pub fn handle_key_event(&mut self, action: Option<KeyAction>) {
        match action {
            Some(action) => match action {
                KeyAction::Single(a) => self.handle_single_action(a),
                KeyAction::Multiple(_) => todo!(),
                KeyAction::Nested(_) => todo!(),
                KeyAction::Repeating(_, _) => todo!(),
            },
            None => todo!(),
        }
    }

    pub fn new(config: Config, path: String) -> anyhow::Result<Self> {
        let out: Stdout = stdout();

        match Window::new(path.clone()) {
            Err(e) => return Err(e),
            Ok(w) => Ok(Self {
                out,
                config,
                current_buffer: w,
                alt_buffers: Vec::new(),
            }),
        }
    }

    fn handle_normal_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Char(c) => {
                    let normal = self.config.keys.normal.clone();
                    let action = normal.get(&format!("{c}")).cloned();
                    match action {
                        Some(_) => self.handle_key_event(action.clone()),
                        None => todo!(),
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn handle_insert_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Char(c) => {}
                KeyCode::Esc => {
                    self.enter_normal_mode();
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn refresh_screen(&mut self) {
        if self.current_buffer.render_buffer {
            self.clear();
            for (i, line) in self.current_buffer.buffer.iter().enumerate() {
                self.out.queue(cursor::MoveTo(0, i as u16)).unwrap();
                self.out.queue(style::Print(format!("{}\r", line))).unwrap();
            }
            self.move_cursor(self.current_buffer.cursor.x, self.current_buffer.cursor.y);
            self.current_buffer.render_buffer = false;
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.clear();
        self.enter_alt_screen();
        self.raw();

        loop {
            self.refresh_screen();
            self.flush();

            let ev = read()?;

            match self.current_buffer.mode {
                Mode::Normal => self.handle_normal_event(ev),
                Mode::Insert => self.handle_insert_event(ev),
            }
        }
    }

    fn handle_single_action(&mut self, a: Action) {
        match a {
            Action::Quit => {
                self.leave_alt_screen();
                self.disable_raw();
                std::process::exit(0);
            }
            Action::MoveUp => self.move_cursor(
                self.current_buffer.cursor.x,
                self.current_buffer.cursor.y - 1,
            ),
            Action::MoveDown => self.move_cursor(
                self.current_buffer.cursor.x,
                self.current_buffer.cursor.y + 1,
            ),
            Action::MoveLeft => self.move_cursor(
                self.current_buffer.cursor.x - 1,
                self.current_buffer.cursor.y,
            ),
            Action::MoveRight => self.move_cursor(
                self.current_buffer.cursor.x + 1,
                self.current_buffer.cursor.y,
            ),
            Action::InsertMode => self.enter_insert_mode(),
            Action::NormalMode => self.enter_normal_mode(),
            Action::DeleteUnderCursor => todo!(),
        }
    }
}
