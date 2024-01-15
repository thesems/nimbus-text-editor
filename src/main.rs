use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use std::io::{self, stdin, stdout, Write};

fn main() -> Result<(), io::Error> {
    let mut stdout = stdout().into_raw_mode()?;

    let stdin = stdin();
    for key_opt in stdin.keys() {
        let key = key_opt.unwrap();
        match key {
            Key::Char(c) => {
                if c == 'q' {
                    return Ok(());
                }
                print!("{}", c);
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
    Ok(())
}
