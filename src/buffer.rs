use std::fs::{self, File};
use std::io::{ErrorKind, Read};
use std::{fmt, io};

use crate::piece_table::PieceTable;
use crate::position::Position;

#[derive(Debug)]
struct FilePathUndefined;

impl fmt::Display for FilePathUndefined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File path not defined.")
    }
}

#[derive(Default)]
pub struct Buffer {
    piece_table: PieceTable,
    file_path: Option<String>,
}
impl Buffer {
    pub fn new() -> Buffer {
        Self::from_string("".to_string())
    }

    pub fn from_string(contents: String) -> Buffer {
        Buffer {
            piece_table: PieceTable::from_string(contents),
            file_path: None,
        }
    }

    pub fn from_file(file_path: &str) -> std::io::Result<Buffer> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Buffer {
            piece_table: PieceTable::from_string(contents),
            file_path: Some(file_path.to_string()),
        })
    }

    pub fn file_path(&self) -> Option<&str> {
        if let Some(path) = self.file_path.as_ref() {
            return Some(path.as_str());
        }
        None
    }

    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path);
    }

    pub fn save_file(&self) -> std::io::Result<&str> {
        if let Some(path) = self.file_path.as_ref() {
            fs::write(path, self.piece_table.get(0, None))?;
            Ok(path.as_str())
        } else {
            Err(io::Error::new(
                ErrorKind::Other,
                "Variable file_path not set.",
            ))
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        if let Some(file_path) = &self.file_path {
            return Some(file_path.split('/').last().unwrap());
        }
        None
    }

    pub fn file_extension(&self) -> Option<&str> {
        if let Some(file_name) = &self.file_name() {
            return Some(file_name.split('.').last().unwrap());
        }
        None
    }

    pub fn get(&self, from: &Position, until: Option<&Position>) -> String {
        if let Some(from_offset) = self.piece_table.get_offset_from_position(from) {
            if let Some(until_pos) = until {
                if let Some(until_offset) = self.piece_table.get_offset_from_position(until_pos) {
                    return self.piece_table.get(from_offset, Some(until_offset));
                }
            }
            return self.piece_table.get(from_offset, None);
        }
        self.piece_table.get(0, None)
    }

    pub fn find(&self, text: &str, offset: usize, all: bool) -> Vec<std::ops::Range<usize>> {
        self.piece_table.find(text, offset, all)
    }

    pub fn insert_new_line(&mut self, position: &Position) {
        if let Some(offset) = self.piece_table.get_offset_from_position(position) {
            self.piece_table.insert_new_line(offset);
        } else {
            // TODO: write warning to logs
        }
    }

    pub fn insert(&mut self, text: &str, position: &Position) {
        if let Some(offset) = self.piece_table.get_offset_from_position(position) {
            self.piece_table.insert(text, offset);
        } else {
            // TODO: write warning to logs
        }
    }

    pub fn delete(&mut self, position: &Position, count: usize) {
        if let Some(offset) = self.piece_table.get_offset_from_position(position) {
            self.piece_table.delete(offset, count);
        } else {
            // TODO: write warning to logs
        }
    }

    /// Check if the buffer contains the column for the line. Use 0-based alignment.
    pub fn get_line_length(&self, y: usize) -> usize {
        let y_line_start_res = self
            .piece_table
            .get_offset_from_position(&Position { x: 0, y });
        let next_y_line_start_res = self
            .piece_table
            .get_offset_from_position(&Position { x: 0, y: y + 1 });

        if let Some(y_line_start) = y_line_start_res {
            if let Some(next_y_line_start) = next_y_line_start_res {
                return next_y_line_start
                    .saturating_sub(y_line_start)
                    .saturating_sub(2);
            }
            // TODO: avoid getting full sequence
            // idea: count via piece in reverse
            let content_len = self
                .get(&Position::new(0, 0), None)
                .chars()
                .skip(y_line_start)
                .filter(|x| *x != '\n' && *x != '\r')
                .count();

            return content_len;
        }
        0
    }

    pub fn get_total_lines(&self) -> usize {
        self.piece_table.get_total_lines()
    }

    pub fn get_offset_from_position(&self, position: &Position) -> Option<usize> {
        self.piece_table.get_offset_from_position(position)
    }

    pub fn get_position_from_offset(&self, offset: usize) -> Position {
        self.piece_table.get_position_from_offset(offset)
    }

    pub fn get_debug_status(&self, position: &Position) -> String {
        let debug_offset = self
            .piece_table
            .get_offset_from_position(position)
            .unwrap_or(0);

        format!(
            "nl_data={:?} | nl_add={:?} | offset={} | pieces={:?} | {:?}",
            self.piece_table.line_starts_data,
            self.piece_table.line_starts_add,
            debug_offset,
            self.piece_table.pieces,
            self.piece_table.get(0, None),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_init() {
        let buffer = Buffer::from_string(String::from("This is already in the file."));
        assert_eq!(
            buffer.piece_table.get(0, None),
            "This is already in the file."
        )
    }

    #[test]
    fn test_buffer_get() {
        let buffer =
            Buffer::from_string(String::from("File is read.\r\nThe hero lied.\r\nThe end."));
        assert_eq!(
            buffer.get(&Position::new(0, 1), None),
            "The hero lied.\r\nThe end."
        );
        assert_eq!(
            buffer.get(&Position::new(0, 1), Some(&Position::new(0, 2))),
            "The hero lied.\r\n"
        );
    }

    #[test]
    fn test_buffer_search() {
        let buffer = Buffer::from_string(String::from(
            "File is read.\r\nThe hero lied.\r\nThe end.\r\nThe hero rises once more.",
        ));
        let occurences = buffer.find("hero", 0, false);
        assert_eq!(occurences[0], 19..23);
        assert_eq!(occurences[1], 45..49);
    }

    #[test]
    fn test_get_line_length() {
        let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.".to_string());

        assert_eq!(buffer.get_line_length(0), 13);
        assert_eq!(buffer.get_line_length(1), 14);
        assert_eq!(buffer.get_line_length(2), 8);
        assert_eq!(buffer.get_line_length(3), 0);
        assert_eq!(buffer.get_line_length(4), 0);
    }
}
