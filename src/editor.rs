use std::{
    collections::HashMap,
    io::{stdout, Stdout, Write},
};

use anyhow::Result;
use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use serde::{Deserialize, Serialize};

use crate::config::{Config, KeyAction};
use crate::window::Window;

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
    Save,
    InsertAfter,
    InsertLineAfter,
    InsertLineAbove,
    DeleteLine,
    DeleteWord,
    MoveWordRight,
    MoveWordLeft,
    MoveToStartOfLine,
    MoveToEndOfLine,
    Undo
}

impl Editor {
    fn move_cursor(&mut self, x: u16, y: u16) {
        let buffer_height = {
            if self.current_buffer.buffer.len() as u16 > 0 {
                self.current_buffer.buffer.len() as u16 - 1
            } else {
                0
            }
        };
        if y > buffer_height {
            self.current_buffer.cursor.y = buffer_height;
        } else {
            self.current_buffer.cursor.y = y;
        }

        let mut buffer_width = {
            if self.current_buffer.buffer[self.current_buffer.cursor.y as usize].len() as u16 > 0 {
                self.current_buffer.buffer[self.current_buffer.cursor.y as usize].len() as u16 - 1
            } else {
                0
            }
        };

        if self.current_buffer.mode == Mode::Insert && buffer_width > 0 {
            buffer_width += 1;
        }

        if x > buffer_width {
            self.current_buffer.cursor.x = buffer_width;
        } else {
            self.current_buffer.cursor.x = x;
        }

        self.out
            .queue(cursor::MoveTo(
                self.current_buffer.cursor.x,
                self.current_buffer.cursor.y,
            ))
            .unwrap();
    }

    fn enter_insert_mode(&mut self, after: bool) {
        self.out.queue(SetCursorStyle::BlinkingBar).unwrap();
        self.current_buffer.mode = Mode::Insert;
        if after {
            self.move_cursor(
                self.current_buffer.cursor.x + 1,
                self.current_buffer.cursor.y,
            );
        }
    }

    fn enter_normal_mode(&mut self) {
        if self.current_buffer.mode == Mode::Insert && self.current_buffer.cursor.x > 0 {
            self.move_cursor(self.current_buffer.cursor.x - 1, self.current_buffer.cursor.y);
        } else {
            self.move_cursor(self.current_buffer.cursor.x, self.current_buffer.cursor.y);
        }
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
                KeyAction::Multiple(a) => print!("{:?}", a),
                KeyAction::Nested(a) => self.handle_nested_action(a),
                KeyAction::Repeating(_, _) => (),
            },
            None => (),
        }
    }

    pub fn handle_nested_action(&mut self, actions: HashMap<String, KeyAction>) {
        let event = read().unwrap();

        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(c) => match actions.get(&c.to_string()) {
                    Some(x) => self.handle_key_event(Some(x.clone())),
                    None => (),
                },
                _ => (),
            },
            _ => (),
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
                    let modifier = match modifiers {
                        KeyModifiers::SHIFT => "S-",
                        KeyModifiers::CONTROL => "C-",
                        _ => "",
                    };

                    let normal = self.config.keys.normal.clone();

                    let action = normal.get(&format!("{modifier}{c}")).cloned();
                    match action {
                        Some(_) => self.handle_key_event(action.clone()),
                        None => (),
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_insert_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(c) => {
                    self.current_buffer.insert(c.to_string());
                }
                KeyCode::Esc => {
                    self.enter_normal_mode();
                }
                KeyCode::Up => self.handle_single_action(Action::MoveUp),
                KeyCode::Down => self.handle_single_action(Action::MoveDown),
                KeyCode::Left => self.handle_single_action(Action::MoveLeft),
                KeyCode::Right => self.handle_single_action(Action::MoveRight),
                KeyCode::Backspace => self.handle_single_action(Action::DeleteUnderCursor),
                _ => (),
            },
            _ => (),
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

            self.current_buffer.push_snapshot();
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
            Action::MoveUp => {
                if self.current_buffer.cursor.y > 0 {
                    self.move_cursor(
                        self.current_buffer.cursor.x,
                        self.current_buffer.cursor.y - 1,
                    );
                } else {
                    self.move_cursor(self.current_buffer.cursor.x, self.current_buffer.cursor.y);
                }
            }
            Action::MoveDown => self.move_cursor(
                self.current_buffer.cursor.x,
                self.current_buffer.cursor.y + 1,
            ),
            Action::MoveLeft => {
                if self.current_buffer.cursor.x > 0 {
                    self.move_cursor(
                        self.current_buffer.cursor.x - 1,
                        self.current_buffer.cursor.y,
                    );
                    return;
                } else {
                    self.move_cursor(self.current_buffer.cursor.x, self.current_buffer.cursor.y);
                }
            }
            Action::MoveRight => self.move_cursor(
                self.current_buffer.cursor.x + 1,
                self.current_buffer.cursor.y,
            ),
            Action::InsertMode => self.enter_insert_mode(false),
            Action::InsertAfter => self.enter_insert_mode(true),
            Action::NormalMode => self.enter_normal_mode(),
            Action::InsertLineAfter => self.current_buffer.insert_line_below(),
            Action::InsertLineAbove => self.current_buffer.insert_line_above(),
            Action::DeleteUnderCursor => self.current_buffer.delete_under_cursor(),
            Action::Save => match self.current_buffer.save().map_err(|e| e.to_string()) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            },
            Action::DeleteLine => self.current_buffer.delete_line(),
            Action::MoveToEndOfLine => {
                self.move_cursor(self.current_buffer.width(), self.current_buffer.cursor.y);
                self.enter_insert_mode(true);
            }
            Action::MoveToStartOfLine => {
                self.move_cursor(0, self.current_buffer.cursor.y);
                self.enter_insert_mode(true);
            }
            Action::DeleteWord => self.current_buffer.delete_word(),
            Action::MoveWordLeft => self.move_cursor(self.current_buffer.find_word_start() as u16, self.current_buffer.cursor.y),
            Action::MoveWordRight => self.move_cursor(self.current_buffer.find_word_end() as u16, self.current_buffer.cursor.y),
            Action::Undo => {
                self.current_buffer.do_undo();
            }
        }
    }
}
