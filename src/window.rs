use std::{
    fs::File,
    io::{Read, Write},
};

use crate::editor::Mode;

pub struct Window {
    pub buffer: Vec<String>,
    pub render_buffer: bool,
    pub cursor: Cursor,
    pub mode: Mode,
    pub file_path: String,
}

pub struct Cursor {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Window {
    pub fn new(path: String) -> anyhow::Result<Self> {
        let file = File::open(path.clone());
        let mut buff = String::new();

        match file {
            Err(e) => return Err(anyhow::Error::new(e)),
            Ok(mut f) => {
                f.read_to_string(&mut buff)?;

                return Ok(Self {
                    buffer: buff.lines().map(|l| l.to_string()).collect(),
                    render_buffer: true,
                    cursor: Cursor { x: 0, y: 0 },
                    mode: Mode::Normal,
                    file_path: path,
                });
            }
        }
    }

    pub fn width(&self) -> u16 {
        self.buffer[self.cursor.y as usize].len() as u16
    }

    pub fn line_buffer(&self) -> String {
        self.buffer[self.cursor.y as usize].clone()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let mut file = File::create(self.file_path.clone())?;
        file.write_all(self.buffer.join("\n").as_bytes())?;

        Ok(())
    }

    pub fn insert(&mut self, c: String) {
        c.chars().enumerate().for_each(|(i, c)| {
            self.buffer[self.cursor.y as usize].insert(self.cursor.x as usize + i, c);
            self.cursor.x += 1;
        });
        self.render_buffer = true;
    }

    pub fn find_word_start(&self) -> usize {
        let mut i = self.cursor.x as usize;
        if self.width() == 0 {
            return i;
        }
        while i > 0 {
            if self.buffer[self.cursor.y as usize]
                .chars()
                .nth(i - 1)
                .unwrap()
                .is_alphanumeric()
            {
                i -= 1;
            } else {
                return i;
            }
        }
        i
    }

    pub fn find_word_end(&self) -> usize {
        let mut i = self.cursor.x as usize;
        if self.width() <= 1 {
            return 0;
        }
        if i == self.buffer[self.cursor.y as usize].len() - 1 {
            return i;
        }
        while i < self.buffer[self.cursor.y as usize].len() - 1 {
            if self.buffer[self.cursor.y as usize]
                .chars()
                .nth(i + 1)
                .unwrap()
                .is_alphanumeric()
            {
                i += 1;
            } else {
                return i;
            }
        }
        i
    }

    pub fn delete_word(&mut self) {
        match self.width() {
            0 | 1 => {
                self.buffer[self.cursor.y as usize].clear();
                self.cursor.x = 0;
            }
            _ => {
                let start = self.find_word_start();
                let end = self.find_word_end();
                self.buffer[self.cursor.y as usize].replace_range(start..=end, "");
                self.cursor.x = start as u16;
            }
        }
        self.render_buffer = true;
    }

    pub fn insert_line_below(&mut self) {
        self.buffer
            .insert((self.cursor.y + 1) as usize, String::new());
        self.cursor.y += 1;
        self.cursor.x = 0;
        self.render_buffer = true;
    }

    pub fn insert_line_above(&mut self) {
        self.buffer.insert((self.cursor.y) as usize, String::new());
        self.cursor.x = 0;
        self.render_buffer = true;
    }

    pub fn delete_line(&mut self) {
        self.buffer.remove(self.cursor.y as usize);
        self.render_buffer = true;
    }

    pub fn delete_under_cursor(&mut self) {
        if self.cursor.x as usize >= self.buffer[self.cursor.y as usize].len() {
            return;
        }

        self.buffer[self.cursor.y as usize].remove(self.cursor.x as usize);
        self.render_buffer = true;
    }
}
