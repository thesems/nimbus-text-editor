use nimbus_text_editor::{buffer::Buffer, Position};

#[test]
fn test_buffer_init() {
    let buffer = Buffer::from_string(String::from("This is already in the file."));
    assert_eq!(buffer.get(), "This is already in the file.")
}

#[test]
fn test_get_offset_from_position_1() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);

    assert_eq!(buffer.data, "File is read.\r\nThe hero lied.\r\nThe end.");
    assert_eq!(buffer.get(), "File is read.\r\nThe hero lied.\r\nThe end.");

    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 3, y: 0 }),
        Some(3)
    );
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 0, y: 1 }),
        Some(15)
    );
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 3, y: 1 }),
        Some(18)
    );
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 0, y: 2 }),
        Some(31)
    );
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 7, y: 2 }),
        Some(38)
    );
    assert_eq!(
        buffer.get_offset_from_position(&Position { x: 7, y: 3 }),
        None
    );
}

#[test]
fn test_find_piece_from_offset_2() {
    let file = String::from("File is read.\r\nThe hero lied.");
    let buffer = Buffer::from_string(file);
   
    assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(22).unwrap().0, 0);
    assert!(buffer.find_piece_from_offset(999).is_none());
}

#[test]
fn test_get_line_length() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);

    assert_eq!(buffer.get_line_length(0), 13);
    assert_eq!(buffer.get_line_length(1), 14);
    assert_eq!(buffer.get_line_length(2), 8);
    assert_eq!(buffer.get_line_length(3), 0);
    assert_eq!(buffer.get_line_length(4), 0);
}

#[test]
fn test_is_valid_column() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);

    assert!(buffer.is_valid_column(&Position { x: 0, y: 0 }));
    assert!(buffer.is_valid_column(&Position { x: 13, y: 0 }));
    assert!(buffer.is_valid_column(&Position { x: 0, y: 1 }));
    assert!(buffer.is_valid_column(&Position { x: 14, y: 1 }));
    assert!(buffer.is_valid_column(&Position { x: 0, y: 2 }));
    assert!(buffer.is_valid_column(&Position { x: 8, y: 2 }));
    assert!(!buffer.is_valid_column(&Position { x: 0, y: 3 }));
    
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.\r\n");
    let buffer = Buffer::from_string(file);
    
    assert!(buffer.is_valid_column(&Position { x: 0, y: 3 }));
}

#[test]
fn test_is_valid_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);

    assert!(buffer.is_valid_line(0));
    assert!(buffer.is_valid_line(1));
    assert!(buffer.is_valid_line(2));
    assert!(!buffer.is_valid_line(3));
    assert!(!buffer.is_valid_line(4));
    
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.\r\n");
    let buffer = Buffer::from_string(file);

    assert!(buffer.is_valid_line(3));
    assert!(!buffer.is_valid_line(4));
}

#[test]
fn test_get_total_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.");
    let buffer = Buffer::from_string(file);
    assert_eq!(buffer.get_total_lines(), 3);
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
    assert_eq!(buffer.get(), "File is read.\r\nThe hero has lied.\r\nThe end.");
}

#[test]
fn test_buffer_insert_middle_2nd_line() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("Third line.\r\n", 31);
    assert_eq!(buffer.get(), "File is read.\r\nThe hero lied.\r\nThird line.\r\n");
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
fn test_find_piece_from_offset_3() {
    let file = String::from("File is read.\r\nThe hero lied.");
    let mut buffer = Buffer::from_string(file);
    buffer.insert("has ", 24);

    assert_eq!(buffer.get(), "File is read.\r\nThe hero has lied.");
    assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(24).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(25).unwrap().0, 1);
    assert_eq!(buffer.find_piece_from_offset(30).unwrap().0, 2);
    assert!(buffer.find_piece_from_offset(34).is_none());
    
    buffer.insert_new_line(5);
    assert_eq!(buffer.get(), "File \r\nis read.\r\nThe hero has lied.");
    assert_eq!(buffer.find_piece_from_offset(4).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(5).unwrap().0, 0);
    assert_eq!(buffer.find_piece_from_offset(6).unwrap().0, 1);
}

#[test]
fn test_delete() {
    let file = String::from("File is read.\r\nThe hero lied.\r\n");
    let mut buffer = Buffer::from_string(file);

    buffer.delete(&Position { x: 3, y: 0 }, 1);
    assert_eq!(buffer.get(), "Fil is read.\r\nThe hero lied.\r\n");

    buffer.delete(&Position { x: 14, y: 0 }, 16);
    assert_eq!(buffer.get(), "Fil is read.\r\n");
}

#[test]
fn test_complex_1() {
    let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.\r\n");
    let mut buffer = Buffer::from_string(file);

    buffer.delete(&Position { x: 14, y: 1 }, 2);
    assert_eq!(buffer.get(), "File is read.\r\nThe hero lied.The end.\r\n");

    assert_eq!(buffer.get_line_length(1), 22);
    assert_eq!(buffer.get_total_lines(), 3);

    assert!(buffer.is_valid_column(&Position { x: 22, y: 1 }));
    assert!(!buffer.is_valid_column(&Position { x: 23, y: 1 }));
    assert!(buffer.is_valid_line(1));
    assert!(buffer.is_valid_line(2));
    assert!(!buffer.is_valid_line(3));
}
