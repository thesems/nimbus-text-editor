use nimbus_text_editor::editor::Editor;
use std::{env, io::Error, process};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();
 
    let file_name = match args.len() {
        0 => None,
        1 => Some(args[0].as_str()),
        2.. => {
            eprintln!("Too many arguments!");
            eprintln!("Usage: program [file_name]?");
            process::exit(1);
        }
    };

    let mut editor = Editor::new(file_name)?;
    editor.main_loop();
    Ok(())
}
