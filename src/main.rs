use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

use std::io::{stdout, Stdout, Write};

enum Mode {
    Normal,
    Insert,
}

enum Command {
    InsertMode = Char('i'),
    Normal,
}

struct Cursor {
    x: u16,
    y: u16,
}

struct Editor {
    out: Stdout,
    cursor: Cursor,
    mode: Mode,
}

trait NewTrait {
    fn handle_insert_key_event_with_modifier(&mut self, event: KeyEvent);
    fn handle_normal_key_event_with_modifier(&mut self, event: KeyEvent);
    fn handle_insert_key_event(&mut self, event: KeyEvent);
    fn handle_normal_key_event(&mut self, event: KeyEvent);
    fn handle_key_event(&mut self, event: KeyEvent);
    fn leave_alt_screen(&mut self);
    fn flush(&mut self);
    fn move_cursor(&mut self, x: u16, y: u16);
    fn enter_insert_mode(&mut self);
    fn clear(&mut self);
    fn enter_alt_screen(&mut self);
}

impl NewTrait for Editor {
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
}

fn main() -> anyhow::Result<()> {
    let mut editor = Editor {
        out: stdout(),
        cursor: Cursor { x: 0, y: 0 },
        mode: Mode::Normal,
    };
    // Disable default terminal before like input handling and Ctrl modifiers
    terminal::enable_raw_mode()?;

    // Clear the terminal
    editor.clear();

    // Enter alt screen
    editor.enter_alt_screen();

    // Position cursor at origin
    editor.move_cursor(0, 0);

    loop {
        // Run any queued commands
        editor.flush();
        match read()? {
            Event::Key(event) => println!("{:?}", event),
            _ => {}
        }

        break;
    }
    editor.leave_alt_screen();

    return Ok(());
}
