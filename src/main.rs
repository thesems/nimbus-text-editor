use nimbus_text_editor::{buffer::Buffer, editor::Editor};
use std::{env, io::Error, process};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut x: u64 = 0;

    let file_name = match args.len() {
        0 => None,
        1 => Some(args[0].as_str()),
        2.. => {
            eprintln!("Too many arguments!");
            eprintln!("Usage: program [file_name]?");
            process::exit(1);
        }
    };

    // let file_name = Some("test.txt");

    let buffer = match file_name {
        Some(path) => Buffer::from_file(path)?,
        None => Buffer::default(),
    };

    let mut editor = Editor::new(buffer)?;
    editor.run();
    Ok(())
}
