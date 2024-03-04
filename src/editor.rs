#[cfg(test)]
mod tests;

use std::io::{stdin, Error};
use termion::event::Key;
use crate::{buffer::Buffer, position::Position, terminal::Terminal};

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    current_line_length: usize,
    buffer: Buffer,
    status: String,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Result<Editor, Error> {
        let editor = Editor {
            terminal: Terminal::new()?,
            cursor_position: Position::default(),
            current_line_length: buffer.get_line_length(0),
            buffer,
            status: String::new(),
        };
        Ok(editor)
    }

    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.current_line_length = buffer.get_line_length(self.cursor_position.y);
        self.cursor_position = Position::default();
        self.buffer = buffer;
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
            self.status = format!("col {} | row {} | line {}", self.cursor_position.x, self.cursor_position.y, self.current_line_length);
            self.terminal.goto(&Position {
                x: 0,
                y: self.terminal.size().1 as usize - 1,
            });
            self.terminal.write(&self.status);

            // Draw buffer
            self.terminal.goto(&Position::default());
            self.terminal.write(&self.buffer.get());

            // Position cursor
            self.terminal.goto(&Position::new(
                std::cmp::min(self.cursor_position.x, self.current_line_length),
                self.cursor_position.y,
            ));

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
                    self.current_line_length = self.buffer.get_line_length(self.cursor_position.y);
                }
                Key::Ctrl(c) => {
                    if c == 'q' {
                        self.terminal.clear();
                        break;
                    }
                }
                Key::Backspace => {
                    let offset = self
                        .buffer
                        .get_offset_from_position(&self.cursor_position)
                        .unwrap();

                    if self.cursor_position.x > 0 {
                        self.cursor_position.x -= 1;
                        self.buffer.delete(offset - 1, 1);
                    } else if self.cursor_position.y > 0 {
                        let line_len = self.buffer.get_line_length(self.cursor_position.y - 1);
                        self.cursor_position.x = line_len;
                        self.cursor_position.y -= 1;
                        self.buffer.delete(offset - 2, 2);
                    } else {
                        // empty
                        continue;
                    }
                    
                    self.current_line_length = self.buffer.get_line_length(self.cursor_position.y);
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
                    if self.is_valid_column(&new_position) {
                        self.cursor_position.x += 1;
                    }
                }
                Key::Up => {
                    if self.cursor_position.y > 0 {
                        self.cursor_position.y -= 1;
                    }
                    self.current_line_length = self.buffer.get_line_length(self.cursor_position.y);
                }
                Key::Down => {
                    if self.is_valid_line(self.cursor_position.y + 1) {
                        self.cursor_position.y += 1;
                    }
                    self.current_line_length = self.buffer.get_line_length(self.cursor_position.y);
                }
                _ => {
                    dbg!(&key);
                }
            }
        }
    }

    /// Check if the buffer contains the line. Use 0-based alignment.
    pub fn is_valid_line(&self, line: usize) -> bool {
        line < self.buffer.get_total_lines()
    }

    /// Check if the buffer contains the column for the line. Use 0-based alignment.
    pub fn is_valid_column(&self, position: &Position) -> bool {
        if position.y + 1 == self.buffer.get_total_lines()
            && position.x == 0
            && self.current_line_length == 0
        {
            return true;
        }
        if self.current_line_length == 0 || position.x > self.current_line_length {
            return false;
        }
        true
    }
}
