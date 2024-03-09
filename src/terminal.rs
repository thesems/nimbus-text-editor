use std::io::{stdout, Error, Stdin, Stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::position::Position;

pub struct Terminal {
    size: (u16, u16),
    stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, Error> {
        let size = termion::terminal_size()?;
        Ok(Terminal {
            size,
            stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn read_key(&self, stdin: &Stdin) -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin.lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn goto(&self, position: &Position) {
        let (x, y) = position.get_terminal();
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn write(&self, buffer: &str) {
        print!("{}", buffer);
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn write_with_color(&self, buffer: &str, color: &dyn color::Color) {
        print!("{}{}{}", color::Fg(color), buffer, color::Fg(color::Reset));
    }

    pub fn show_cursor(&self) {
        print!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide);
    }
}
