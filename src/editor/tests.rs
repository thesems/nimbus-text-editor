use crate::{buffer::Buffer, editor::Editor, position::Position};

#[test]
fn test_is_valid_column() {
    let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.".to_string());
    let editor = Editor::new(buffer).unwrap();

    assert!(editor.is_valid_column(&Position::new(0, 0)));
    assert!(editor.is_valid_column(&Position::new(13, 0)));
    assert!(editor.is_valid_column(&Position::new(0, 1)));
    assert!(editor.is_valid_column(&Position::new(14, 1)));
    assert!(editor.is_valid_column(&Position::new(0, 2)));
    assert!(editor.is_valid_column(&Position::new(8, 2)));

    // assert!(!buffer.is_valid_column(&Position { x: 0, y: 3 }));
    // let file = String::from("File is read.\r\nThe hero lied.\r\nThe end.\r\n");
    // let buffer = Buffer::from_string(file);
    // assert!(buffer.is_valid_column(&Position { x: 0, y: 3 }));
}

#[test]
fn test_is_valid_line() {
    let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.".to_string());
    let editor = Editor::new(buffer).unwrap();

    assert!(editor.is_valid_line(0));
    assert!(editor.is_valid_line(1));
    assert!(editor.is_valid_line(2));
    assert!(!editor.is_valid_line(3));
    assert!(!editor.is_valid_line(4));

    let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.\r\n".to_string());
    let editor = Editor::new(buffer).unwrap();

    assert!(editor.is_valid_line(3));
    assert!(!editor.is_valid_line(4));
}

#[test]
fn test_editor_complex_1() {
    let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.\r\n".to_string());
    let mut editor = Editor::new(buffer).unwrap();

    editor.buffer.delete(29, 2);
    assert_eq!(editor.buffer.get(), "File is read.\r\nThe hero lied.The end.\r\n");

    assert_eq!(editor.buffer.get_line_length(1), 22);
    assert_eq!(editor.buffer.get_total_lines(), 3);

    assert!(editor.is_valid_column(&Position { x: 22, y: 1 }));
    assert!(!editor.is_valid_column(&Position { x: 23, y: 1 }));
    assert!(editor.is_valid_line(1));
    assert!(editor.is_valid_line(2));
    assert!(!editor.is_valid_line(3));
}
