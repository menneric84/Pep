use std::{fs::File, io::Read};

use crate::editor::Mode;

pub struct Window {
    pub buffer: Vec<String>,
    pub render_buffer: bool,
    pub cursor: Cursor,
    pub mode: Mode,
    pub file_path: String

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
                    file_path: path
                })
            }
        }
    }
}
