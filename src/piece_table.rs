use std::ops::Range;

use crate::piece::{Piece, Source};
use crate::position::Position;

#[derive(Default)]
pub struct PieceTable {
    pub data: String,
    pub add: String,
    pub pieces: Vec<Piece>,
    pub line_starts_data: Vec<usize>,
    pub line_starts_add: Vec<usize>,
}
impl PieceTable {
    pub fn from_string(mut data: String) -> PieceTable {
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

        PieceTable {
            data,
            add: String::new(),
            pieces: vec![Piece::new(Source::Data, 0, length)],
            line_starts_data: new_lines,
            line_starts_add: vec![],
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

    /// Get the visible contents of the buffer. Specify an starting offset
    /// and optionally the length of the contents. If None is passed,
    /// it obtains everything until the end.
    ///
    /// Runtime: O(n), where n = # of pieces
    ///
    pub fn get(&self, offset: usize, until_offset: Option<usize>) -> String {
        let until = until_offset.unwrap_or(0);
        let mut data = String::new();

        let mut counted_start = 0;
        let mut counted_len = 0;

        for piece in self.pieces.iter() {
            let mut range = piece.offset..piece.offset + piece.length;

            if counted_start + piece.length < offset {
                counted_start += piece.length;
                continue;
            }
            if counted_start < offset {
                range.start = offset - counted_start;
                counted_start += range.start;
            }

            if until_offset.is_some() && counted_len + piece.length > until {
                range.end = piece.offset + std::cmp::min(piece.length, until - counted_len);
            }

            if piece.source == Source::Data {
                data.push_str(&self.data[range]);
            } else {
                data.push_str(&self.add[range]);
            }
            counted_len += piece.length;
        }
        data
    }

    pub fn find(&self, text: &str, offset: usize) -> Vec<Range<usize>> {
        let mut found = vec![];

        let mut counted_start = 0;
        for piece in self.pieces.iter() {
            let mut range = piece.offset..piece.offset + piece.length;

            if counted_start + piece.length < offset {
                counted_start += piece.length;
                continue;
            }
            if counted_start < offset {
                range.start = offset - counted_start;
                counted_start += range.start;
            }

            let data = match piece.source {
                Source::Data => &self.data[range],
                Source::Add => &self.add[range],
            };

            if let Some(idx) = data.find(text) {
                found.push(idx..idx+text.len());
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_init() {
        let buffer = PieceTable::from_string(String::from("This is already in the file."));
        assert_eq!(buffer.get(0, None), "This is already in the file.")
    }

    #[test]
    fn test_buffer_get() {
        let buffer =
            PieceTable::from_string(String::from("File is read.\r\nThe hero lied.\r\nThe end."));
        assert_eq!(buffer.get(15, Some(29)), "The hero lied.")
    }

    #[test]
    fn test_get_offset_from_position_unmodified_buffer() {
        let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
        let buffer = PieceTable::from_string(file);

        assert_eq!(buffer.data, "File is read.\r\nThe hero lied.\r\nThe end.");
        assert_eq!(
            buffer.get(0, None),
            "File is read.\r\nThe hero lied.\r\nThe end."
        );

        assert_eq!(
            buffer.get_offset_from_position(&Position::new(3, 0)),
            Some(3)
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position::new(0, 1)),
            Some(15)
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position::new(3, 1)),
            Some(18)
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position::new(0, 2)),
            Some(31)
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position::new(7, 2)),
            Some(38)
        );
        assert_eq!(buffer.get_offset_from_position(&Position::new(0, 3)), None);
        assert_eq!(buffer.get_offset_from_position(&Position::new(7, 3)), None);
    }

    #[test]
    fn test_find_piece_from_offset_unmodified_buffer() {
        let file = String::from("File is read.\r\nThe hero lied.");
        let buffer = PieceTable::from_string(file);

        assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
        assert_eq!(buffer.find_piece_from_offset(22).unwrap().0, 0);
        assert!(buffer.find_piece_from_offset(999).is_none());
    }

    #[test]
    fn test_find_piece_from_offset_modified_buffer() {
        let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
        let mut buffer = PieceTable::from_string(file);
        buffer.insert("has ", 24);
        buffer.delete(33, 2);
        buffer.insert_new_line(5);

        assert_eq!(
            buffer.get(0, None),
            "File \r\nis read.\r\nThe hero has lied.The end."
        );
        assert_eq!(buffer.find_piece_from_offset(4).unwrap().0, 0);
        assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
        assert_eq!(buffer.find_piece_from_offset(6).unwrap().0, 1);
        assert_eq!(buffer.find_piece_from_offset(13).unwrap().0, 2);
        assert_eq!(buffer.find_piece_from_offset(14).unwrap().0, 2);
        assert_eq!(buffer.find_piece_from_offset(30).unwrap().0, 3);
        assert!(buffer.find_piece_from_offset(50).is_none());
    }

    #[test]
    fn test_find_piece_from_offset_modified_buffer_2() {
        let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
        let mut buffer = PieceTable::from_string(file);
        buffer.delete(13, 2);

        assert_eq!(
            buffer.get(0, None),
            "File is read.The hero lied.\r\nThe end."
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position::new(0, 1)),
            Some(29)
        );
    }

    #[test]
    fn test_buffer_insert() {
        let file = String::from("This is already in the file.");
        let file_len = file.len();
        let mut buffer = PieceTable::from_string(file);

        buffer.insert("New text appended.", file_len);
        assert_eq!(
            buffer.get(0, None),
            "This is already in the file.New text appended."
        );
    }

    #[test]
    fn test_buffer_insert_middle() {
        let file = String::from("File is read.");
        let mut buffer = PieceTable::from_string(file);
        buffer.insert("not ", 8);
        assert_eq!(buffer.get(0, None), "File is not read.");
    }

    #[test]
    fn test_buffer_insert_new_line() {
        let file = String::from("File is read.\r\nThe hero lied.\r\n");
        let mut buffer = PieceTable::from_string(file);

        assert_eq!(
            buffer.get_offset_from_position(&Position { x: 5, y: 1 }),
            Some(20)
        );

        buffer.insert_new_line(5);
        assert_eq!(
            buffer.get(0, None),
            "File \r\nis read.\r\nThe hero lied.\r\n"
        );
        assert_eq!(
            buffer.get_offset_from_position(&Position { x: 4, y: 1 }),
            Some(11)
        );

        buffer.insert("\r\n", 10);
        assert_eq!(
            buffer.get(0, None),
            "File \r\nis \r\nread.\r\nThe hero lied.\r\n"
        );

        buffer.insert("\r\n", 5);
        buffer.insert("\r\n", 5);
        buffer.insert("\r\n", 5);
        assert_eq!(
            buffer.get(0, None),
            "File \r\n\r\n\r\n\r\nis \r\nread.\r\nThe hero lied.\r\n"
        );
    }

    #[test]
    fn test_buffer_insert_middle_multiple_lines() {
        let file = String::from("File is read.\r\nThe hero lied.");
        let mut buffer = PieceTable::from_string(file);
        buffer.insert("has ", 24);
        assert_eq!(buffer.get(0, None), "File is read.\r\nThe hero has lied.");
        buffer.insert_new_line(33);
        buffer.insert("The end.", 35);
        assert_eq!(
            buffer.get(0, None),
            "File is read.\r\nThe hero has lied.\r\nThe end."
        );
    }

    #[test]
    fn test_buffer_insert_middle_2nd_line() {
        let file = String::from("File is read.\r\nThe hero lied.\r\n");
        let mut buffer = PieceTable::from_string(file);
        buffer.insert("Third line.\r\n", 31);
        assert_eq!(
            buffer.get(0, None),
            "File is read.\r\nThe hero lied.\r\nThird line.\r\n"
        );
    }

    #[test]
    fn test_buffer_insert_middle_2nd_line_sequence() {
        let file = String::from("File is read.\r\nThe hero lied.");
        let mut buffer = PieceTable::from_string(file);
        buffer.insert("h", 24);
        buffer.insert("a", 25);
        buffer.insert("s", 26);
        buffer.insert(" ", 27);
        assert_eq!(buffer.get(0, None), "File is read.\r\nThe hero has lied.");
    }

    #[test]
    fn test_delete() {
        let file = String::from("File is read.\r\nThe hero lied.\r\n");
        let mut buffer = PieceTable::from_string(file);

        buffer.delete(3, 1);
        assert_eq!(buffer.get(0, None), "Fil is read.\r\nThe hero lied.\r\n");

        buffer.delete(14, 16);
        assert_eq!(buffer.get(0, None), "Fil is read.\r\n");
    }

    #[test]
    fn test_delete_insert_alternate() {
        let file = String::from("File is read.\r\nThe hero lied.\r\n");
        let mut buffer = PieceTable::from_string(file);

        buffer.delete(7, 8);
        buffer.insert_new_line(7);
        buffer.insert(".", 7);
        assert_eq!(buffer.get(0, None), "File is.\r\nThe hero lied.\r\n");
    }

    #[test]
    fn test_get_total_line() {
        let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
        let buffer = PieceTable::from_string(file);
        assert_eq!(buffer.get_total_lines(), 3);
    }

    #[test]
    fn test_delete_2() {
        let mut buffer = PieceTable::from_string(
            "File is read.\r\nThe hero lied.\r\nThe end.\r\nBensu.\r\nHello.\r\nlo.\r\n"
                .to_string(),
        );

        buffer.delete(57, 5);
        assert_eq!(
            buffer.get(0, None),
            "File is read.\r\nThe hero lied.\r\nThe end.\r\nBensu.\r\nHello.\r\n".to_string()
        );
    }
}
