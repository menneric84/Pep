use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    event::{self, read, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use serde::{Deserialize, Serialize};

use crate::config::{self, Config};

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

    pub fn new(config: Config) -> anyhow::Result<Self> {
        let mut out: Stdout = stdout();

        Ok(Self {
            out,
            cursor: Cursor { x: 0, y: 0 },
            mode: Mode::Normal,
            config,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.clear();
        self.flush();

        let ev = read()?;
        let normal = self.config.keys.normal.clone();

        let key_action = match ev {
            event::Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                let key = format!("{code:?}");

                let key = match modifiers {
                    KeyModifiers::CONTROL => format!("Ctrl-{key}"),
                    KeyModifiers::ALT => format!("Alt-{key}"),
                    _ => key,
                };

                println!("{:?}", key);
                normal.get(&key).cloned()
            }
            event::Event::FocusGained => todo!(),
            event::Event::FocusLost => todo!(),
            event::Event::Mouse(_) => todo!(),
            event::Event::Paste(_) => todo!(),
            event::Event::Resize(_, _) => todo!(),
        }?;

        match key_action {
            config::KeyAction::Single(action) => {
                self.handle_key_event(action); 

            },
            config::KeyAction::Multiple(_) => todo!(),
            config::KeyAction::Nested(_) => todo!(),
            config::KeyAction::Repeating(_, _) => todo!(),
        }


        let ev = read()?;
        Ok(())
    }
}
