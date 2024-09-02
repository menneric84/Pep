use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use serde::{Deserialize, Serialize};

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
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    Quit(bool),
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

    fn flush(&mut self) {
        self.out.flush().unwrap();
    }

    fn handle_normal_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('i') => {
                self.enter_insert_mode();
            }
            _ => {}
        }
    }

    fn handle_normal_key_event_with_modifier(&mut self, event: KeyEvent) {
        match event.code {
            _ => {}
        }
    }

    fn handle_insert_key_event(&mut self, event: KeyEvent) {
        match event.code {
            _ => {}
        }
    }

    fn handle_insert_key_event_with_modifier(&mut self, event: KeyEvent) {
        match event.code {
            _ => {}
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Normal => {
                if crossterm::event::KeyModifiers::is_empty(&event.modifiers) {
                    self.handle_normal_key_event(event);
                } else {
                    self.handle_normal_key_event_with_modifier(event);
                }
            }
            Mode::Insert => {
                if event.modifiers.is_empty() {
                    self.handle_insert_key_event(event);
                } else {
                    self.handle_insert_key_event_with_modifier(event);
                }
            }
        }
    }

    pub fn new() -> anyhow::Result<Self> {
        let mut out: Stdout = stdout();
        Ok(Self {
            out,
            cursor: Cursor { x: 0, y: 0 },
            mode: Mode::Normal,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.enter_alt_screen();
        self.clear();
        self.flush();


        self.leave_alt_screen();
        Ok(())
    }
}
