use std::io::{stdout, Stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{self, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use serde::{Deserialize, Serialize};

use crate::config::{Config, KeyAction};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

struct Cursor {
    x: u16,
    y: u16,
}

pub struct Editor {
    out: Stdout,
    cursor: Cursor,
    mode: Mode,
    config: Config,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Editor {
    fn move_cursor(&mut self, x: u16, y: u16) {
        self.cursor.x = x;
        self.cursor.y = y;
        self.out.queue(cursor::MoveTo(x, y)).unwrap();
    }

    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
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

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }


    pub fn handle_key_event( &mut self, action: Option<KeyAction>) {
        match action {
            Some(action) => {
                match action {
                    KeyAction::Single(a) => self.handle_single_action(a),
                    KeyAction::Multiple(_) => todo!(),
                    KeyAction::Nested(_) => todo!(),
                    KeyAction::Repeating(_, _) => todo!(),
                }
            }
            None => todo!(),
        }
    }
    

    pub fn new(config: Config) -> anyhow::Result<Self> {
        let out: Stdout = stdout();

        Ok(Self {
            out,
            cursor: Cursor { x: 0, y: 0 },
            mode: Mode::Normal,
            config,
        })
    }

    fn handle_normal_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent{code, modifiers, ..}) => {
                match code {
                    KeyCode::Char(c) => {
                        
                        let normal = self.config.keys.normal.clone();
                        let action = normal.get(&format!("{c}")).cloned();
                        match action {
                            Some(_) => self.handle_key_event(action.clone()),
                            None => todo!(),
                        }


                    }
                    _ => todo!()
                }
            }
            _ => todo!(),
        }

    }

    pub fn run(&mut self) -> Result<()> {
        self.clear();
        self.enter_alt_screen();
        self.raw();

        loop {
        self.flush();

        let ev = read()?;

        match self.mode {
            Mode::Normal => self.handle_normal_event(ev),
            Mode::Insert => todo!(),
        }
        }
    }

    fn handle_single_action(&mut self, a: Action) {
        match a {
            Action::Quit => std::process::exit(0),
            Action::MoveUp => self.move_cursor(self.cursor.x, self.cursor.y - 1),
            Action::MoveDown => self.move_cursor(self.cursor.x, self.cursor.y + 1),
            Action::MoveLeft => self.move_cursor(self.cursor.x - 1, self.cursor.y),
            Action::MoveRight => self.move_cursor(self.cursor.x + 1, self.cursor.y),
        }
    }
}
