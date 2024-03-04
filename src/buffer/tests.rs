use crate::{buffer::Buffer, position::Position};

#[test]
fn test_buffer_init() {
    let buffer = Buffer::from_string(String::from("This is already in the file."));
    assert_eq!(buffer.get(), "This is already in the file.")
}

#[test]
fn test_get_offset_from_position_unmodified_buffer() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);

    assert_eq!(buffer.data, "File is read.\r\nThe hero lied.\r\nThe end.");
    assert_eq!(buffer.get(), "File is read.\r\nThe hero lied.\r\nThe end.");

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
    let buffer = Buffer::from_string(file);

    assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(22).unwrap().0, 0);
    assert!(buffer.find_piece_from_offset(999).is_none());
}

#[test]
fn test_find_piece_from_offset_modified_buffer() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("has ", 24);
    buffer.delete(33, 2);
    buffer.insert_new_line(5);

    assert_eq!(
        buffer.get(),
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
    let mut buffer = Buffer::from_string(file);
    buffer.delete(13, 2);

    assert_eq!(buffer.get(), "File is read.The hero lied.\r\nThe end.");
    assert_eq!(
        buffer.get_offset_from_position(&Position::new(0, 1)),
        Some(29)
    );
}

#[test]
fn test_buffer_insert() {
    let file = String::from("This is already in the file.");
    let file_len = file.len();
    let mut buffer = Buffer::from_string(file);

    buffer.insert("New text appended.", file_len);
    assert_eq!(
        buffer.get(),
        "This is already in the file.New text appended."
    );
}

#[test]
fn test_buffer_insert_middle() {
    let file = String::from("File is read.");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("not ", 8);
    assert_eq!(buffer.get(), "File is not read.");
}

#[test]
fn test_buffer_insert_new_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);

    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 5, y: 1 }),
        Some(20)
    );

    buffer.insert_new_line(5);
    assert_eq!(buffer.get(), "File \r\nis read.\r\nThe hero lied.\r\n");
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 4, y: 1 }),
        Some(11)
    );

    buffer.insert("\r\n", 10);
    assert_eq!(buffer.get(), "File \r\nis \r\nread.\r\nThe hero lied.\r\n");

    buffer.insert("\r\n", 5);
    buffer.insert("\r\n", 5);
    buffer.insert("\r\n", 5);
    assert_eq!(
        buffer.get(),
        "File \r\n\r\n\r\n\r\nis \r\nread.\r\nThe hero lied.\r\n"
    );
}

#[test]
fn test_buffer_insert_middle_multiple_lines() {
    let file = String::from("File is read.\r\nThe hero lied.");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("has ", 24);
    assert_eq!(buffer.get(), "File is read.\r\nThe hero has lied.");
    buffer.insert_new_line(33);
    buffer.insert("The end.", 35);
    assert_eq!(
        buffer.get(),
        "File is read.\r\nThe hero has lied.\r\nThe end."
    );
}

#[test]
fn test_buffer_insert_middle_2nd_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("Third line.\r\n", 31);
    assert_eq!(
        buffer.get(),
        "File is read.\r\nThe hero lied.\r\nThird line.\r\n"
    );
}

#[test]
fn test_buffer_insert_middle_2nd_line_sequence() {
    let file = String::from("File is read.\r\nThe hero lied.");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("h", 24);
    buffer.insert("a", 25);
    buffer.insert("s", 26);
    buffer.insert(" ", 27);
    assert_eq!(buffer.get(), "File is read.\r\nThe hero has lied.");
}

#[test]
fn test_delete() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);

    buffer.delete(3, 1);
    assert_eq!(buffer.get(), "Fil is read.\r\nThe hero lied.\r\n");

    buffer.delete(14, 16);
    assert_eq!(buffer.get(), "Fil is read.\r\n");
}

#[test]
fn test_delete_insert_alternate() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);

    buffer.delete(7, 8);
    buffer.insert_new_line(7);
    buffer.insert(".", 7);
    assert_eq!(buffer.get(), "File is.\r\nThe hero lied.\r\n");
}

#[test]
fn test_get_total_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);
    assert_eq!(buffer.get_total_lines(), 3);
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

#[test]
fn test_delete_2() {
    let mut buffer = Buffer::from_string(
        "File is read.\r\nThe hero lied.\r\nThe end.\r\nBensu.\r\nHello.\r\nlo.\r\n".to_string(),
    );

    buffer.delete(57, 5);
    assert_eq!(
        buffer.get(),
        "File is read.\r\nThe hero lied.\r\nThe end.\r\nBensu.\r\nHello.\r\n".to_string()
    );
}
