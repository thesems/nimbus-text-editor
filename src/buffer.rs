#[cfg(test)]
mod tests;

use std::{fmt, io};
use std::fs::{self, File};
use std::io::{ErrorKind, Read};

use crate::piece::{Piece, Source};
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
    pub data: String,
    pub add: String,
    pub pieces: Vec<Piece>,
    pub line_starts_data: Vec<usize>,
    pub line_starts_add: Vec<usize>,
    file_path: Option<String>,
}
impl Buffer {
    pub fn from_string(mut data: String) -> Buffer {
        if !data.contains("\r\n") {
            data = data.replace('\n', "\r\n");
        }

        let mut new_lines: Vec<usize> = vec![0];
        for (idx, ch) in data.chars().enumerate() {
            if ch == '\n' {
                new_lines.push(idx);
                continue;
            }
        }

        let length = data.len();

        Buffer {
            data,
            add: String::new(),
            pieces: vec![Piece::new(Source::Data, 0, length)],
            line_starts_data: new_lines,
            line_starts_add: vec![],
            file_path: Some(String::new()),
        }
    }

    pub fn from_file(file_path: &str) -> std::io::Result<Buffer> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut buffer = Buffer::from_string(contents);
        buffer.file_path = Some(file_path.to_string());
        Ok(buffer)
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
            fs::write(path, self.get())?;
            Ok(path.as_str())
        } else {
            // return error here
            Err(io::Error::new(
                ErrorKind::Other,
                "Variable file_path not set.",
            ))
        }
    }

    /// Find the piece that contains the logical offset and return its index and
    /// offset into that buffer (data or add).
    /// Given the 0 index in a piece buffer, it will return previous piece index.
    /// This is done, so that insertation is easier in the previous piece.
    pub fn find_piece_from_offset(&self, offset: usize) -> Option<(usize, usize)> {
        let mut remaining_offset = offset;
        for (idx, piece) in self.pieces.iter().enumerate() {
            if remaining_offset <= piece.length {
                return Some((idx, piece.offset + remaining_offset));
            }
            remaining_offset -= piece.length;
        }
        None
    }

    /// Iterate all pieces and count new lines and their offsets, until the
    /// position y matches the count.
    /// Panic if position does not have an offset (out of bounds).
    pub fn get_offset_from_position(&self, position: &Position) -> Option<usize> {
        let mut offset = 0;
        let mut y = position.y;

        if y == 0 {
            return Some(position.x);
        }

        for piece in self.pieces.iter() {
            let line_starts: &Vec<usize> = match piece.source {
                Source::Data => &self.line_starts_data,
                Source::Add => &self.line_starts_add,
            };

            let line_starts_len = line_starts
                .iter()
                .filter(|x| **x != 0 && (piece.offset..piece.offset + piece.length).contains(x))
                .count();

            if line_starts_len == 0 {
                offset += piece.length;
                continue;
            }

            if y > line_starts_len {
                offset += piece.length;
                y -= line_starts_len;
                continue;
            }

            if let Some(line_start) = line_starts
                .iter()
                .filter(|x| **x != 0 && (piece.offset..piece.offset + piece.length).contains(x))
                .nth(y - 1)
            {
                offset += line_start + 1 - piece.offset;
                y = 0;
                break;
            }
        }

        if y > 0 {
            // panic!("Position does not have a valid offset into data.")
            return None;
        }

        Some(offset + position.x)
    }

    pub fn get_total_lines(&self) -> usize {
        let mut total = 0;
        for piece in self.pieces.iter() {
            let line_starts: &Vec<usize> = match piece.source {
                Source::Data => &self.line_starts_data,
                Source::Add => &self.line_starts_add,
            };

            let line_starts_len = line_starts
                .iter()
                .filter(|x| (piece.offset..piece.offset + piece.length).contains(x))
                .count();

            total += line_starts_len;
        }
        total
    }

    pub fn insert_new_line(&mut self, offset: usize) {
        self.insert("\r\n", offset);
    }

    pub fn insert(&mut self, text: &str, offset: usize) {
        if text.is_empty() {
            return;
        }

        let add_buffer_len = self.add.len();
        self.add.push_str(text);
        if text == "\r\n" {
            self.line_starts_add.push(add_buffer_len + 1);
        }

        if let Some((piece_idx, buffer_offset)) = self.find_piece_from_offset(offset) {
            let piece = self.pieces.get_mut(piece_idx).unwrap();

            if piece.source == Source::Add
                && buffer_offset == piece.offset + piece.length
                && piece.offset + piece.length == add_buffer_len
            {
                piece.length += text.len();
                return;
            }

            let new_pieces: Vec<Piece> = [
                Piece::new(piece.source, piece.offset, buffer_offset - piece.offset),
                Piece::new(Source::Add, add_buffer_len, text.len()),
                Piece::new(
                    piece.source,
                    buffer_offset,
                    piece.length - (buffer_offset - piece.offset),
                ),
            ]
            .into_iter()
            .filter(|x| x.length > 0)
            .collect();

            self.pieces.splice(piece_idx..piece_idx + 1, new_pieces);
        } else if self.pieces.is_empty() {
            self.pieces
                .push(Piece::new(Source::Add, add_buffer_len, text.len()));
        }
    }

    pub fn delete(&mut self, offset: usize, count: usize) {
        if count == 0 {
            return;
        }

        let (initial_piece_idx, initial_buffer_offset);
        let (final_piece_idx, final_buffer_offset);

        let mut res = self.find_piece_from_offset(offset);
        if res.is_some() {
            (initial_piece_idx, initial_buffer_offset) = res.unwrap();
        } else {
            return;
        }
        res = self.find_piece_from_offset(offset + count);
        if res.is_some() {
            (final_piece_idx, final_buffer_offset) = res.unwrap();
        } else {
            return;
        }

        if initial_buffer_offset == final_buffer_offset {
            let initial_piece = self.pieces.get_mut(initial_piece_idx).unwrap();
            if initial_buffer_offset == initial_piece.offset {
                // start of piece
                initial_piece.offset += count;
                initial_piece.length -= count;
                return;
            }
            if final_buffer_offset == initial_piece.offset + initial_piece.length {
                // end of piece
                initial_piece.length -= count;
                return;
            }
        }

        let initial_piece = self.pieces.get(initial_piece_idx).unwrap();
        let final_piece = self.pieces.get(final_piece_idx).unwrap();

        let new_pieces: Vec<Piece> = [
            Piece::new(
                initial_piece.source,
                initial_piece.offset,
                initial_buffer_offset - initial_piece.offset,
            ),
            Piece::new(
                final_piece.source,
                final_buffer_offset,
                final_piece.length - (final_buffer_offset - final_piece.offset),
            ),
        ]
        .into_iter()
        .filter(|x| x.length > 0)
        .collect();

        self.pieces
            .splice(initial_piece_idx..final_piece_idx + 1, new_pieces);
    }

    pub fn get(&self) -> String {
        let mut data = String::new();

        for piece in self.pieces.iter() {
            if piece.source == Source::Data {
                data.push_str(&self.data[piece.offset..piece.offset + piece.length]);
            } else {
                data.push_str(&self.add[piece.offset..piece.offset + piece.length]);
            }
        }
        data
    }

    /// Check if the buffer contains the column for the line. Use 0-based alignment.
    pub fn get_line_length(&self, y: usize) -> usize {
        let y_line_start_res = self.get_offset_from_position(&Position { x: 0, y });
        let next_y_line_start_res = self.get_offset_from_position(&Position { x: 0, y: y + 1 });

        if let Some(y_line_start) = y_line_start_res {
            if let Some(next_y_line_start) = next_y_line_start_res {
                return next_y_line_start
                    .saturating_sub(y_line_start)
                    .saturating_sub(2);
            }
            // TODO: avoid getting full sequence
            // idea: count via piece in reverse
            let content_len = self
                .get()
                .chars()
                .skip(y_line_start)
                .filter(|x| *x != '\n' && *x != '\r')
                .count();

            return content_len;
        }
        0
    }
}
