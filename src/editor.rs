use std::io::{stdin, Error};

use termion::event::Key;

use crate::{buffer::Buffer, terminal::Terminal, Position};

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    offset_y: usize,
    buffer: Buffer,
    status: String,
}

impl Editor {
    pub fn new(file_name: Option<&str>) -> Result<Editor, Error> {
        match file_name {
            Some(path) => Ok(Editor {
                terminal: Terminal::new()?,
                cursor_position: Position::default(),
                offset_y: 0,
                buffer: Buffer::from_file(path)?,
                status: String::new(),
            }),
            None => Ok(Editor {
                terminal: Terminal::new()?,
                cursor_position: Position::default(),
                offset_y: 0,
                // buffer: Buffer::new(String::from("File is read.\r\nThe hero lied.\r\n")),
                buffer: Buffer::default(),
                status: String::new(),
            }),
        }
    }
    pub fn main_loop(&mut self) {
        let stdin = stdin();
        loop {
            self.terminal.clear();

            // Debug bar
            let debug_offset = self
                .buffer
                .get_offset_from_position(&self.cursor_position)
                .unwrap_or(0);

            self.buffer.debug = format!(
                "nl_data={:?} | nl_add={:?} | offset={} | pieces={:?} | {:?}", // | data={:?}",
                self.buffer.line_starts_data,
                self.buffer.line_starts_add,
                debug_offset,
                self.buffer.pieces,
                self.buffer.get(),
                // self.buffer.data
            );
            self.terminal.goto(&Position {
                x: 0,
                y: self.terminal.size().1 as usize - 3,
            });
            self.terminal.write(&self.buffer.debug);

            // Status bar
            self.status = format!("{}/{}", self.cursor_position.x, self.cursor_position.y);
            self.terminal.goto(&Position {
                x: 0,
                y: self.terminal.size().1 as usize - 1,
            });
            self.terminal.write(&self.status);

            // Draw buffer
            self.terminal.goto(&Position::default());
            self.terminal.write(&self.buffer.get());
            self.terminal.goto(&self.cursor_position);

            self.terminal.flush();

            let key = self.terminal.read_key(&stdin).unwrap();
            match key {
                Key::Char(c) => {
                    let offset = self
                        .buffer
                        .get_offset_from_position(&self.cursor_position)
                        .unwrap();

                    if c == '\n' {
                        self.buffer.insert_new_line(offset);
                        self.cursor_position.x = 0;
                        self.cursor_position.y += 1;
                    } else {
                        self.buffer.insert(c.to_string().as_str(), offset);
                        self.cursor_position.x += 1;
                    }
                }
                Key::Ctrl(c) => {
                    if c == 'q' {
                        self.terminal.clear();
                        break;
                    }
                }
                Key::Backspace => {
                    if self.cursor_position.x > 0 {
                        self.cursor_position.x -= 1;
                        self.buffer.delete(&self.cursor_position, 1);
                    } else if self.cursor_position.y > 0 {
                        let line_length = self.buffer.get_line_length(self.cursor_position.y - 1);
                        self.cursor_position.x = line_length;
                        self.cursor_position.y -= 1;
                        self.buffer.delete(&self.cursor_position, 2);
                    } else {
                        // empty
                        continue;
                    }
                }
                Key::Left => {
                    if self.cursor_position.x > 0 {
                        self.cursor_position.x -= 1;
                    }
                }
                Key::Right => {
                    let new_position = Position {
                        x: self.cursor_position.x + 1,
                        y: self.cursor_position.y,
                    };
                    if self.buffer.is_valid_column(&new_position) {
                        self.cursor_position.x += 1;
                    }
                }
                Key::Up => {
                    if self.cursor_position.y > 0
                        && self.buffer.is_valid_line(self.cursor_position.y - 1)
                    {
                        self.cursor_position.y -= 1;
                    }
                }
                Key::Down => {
                    if self.buffer.is_valid_line(self.cursor_position.y + 1) {
                        self.cursor_position.y += 1;
                    }
                }
                _ => {
                    dbg!(&key);
                }
            }
        }
    }
}
