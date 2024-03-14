use crate::{
    buffer::Buffer,
    file_extension::FileExtension,
    highlighter::{Highlighter, RustHighlighter},
    position::Position,
    terminal::Terminal,
};
use std::{
    collections::HashMap,
    env,
    io::{stdin, Error, Stdin},
    ops::Range,
};
use termion::{
    color::{self},
    event::Key,
};

#[derive(PartialEq)]
enum EditorMode {
    Normal,
    Insert,
    Command,
}

#[derive(PartialEq)]
enum SearchMode {
    None,
    Incremental,
}

pub struct Editor {
    terminal: Terminal,
    offset_y: usize,
    cursor_position: Position,
    current_line_length: usize,
    status: String,
    command: String,
    mode: EditorMode,
    retain_command_text: bool,
    running: bool,
    untouched: bool,
    debug_bar: bool,
    buffer: Buffer,
    highlighters: HashMap<FileExtension, Box<dyn Highlighter>>,
    extensions: HashMap<String, String>,
    file_extension: String,
    search_mode: SearchMode,
    search_occurences: Vec<Range<usize>>,
    search_occurence_idx: usize,
    search_offset_y: usize,
    search_cursor_position: Position,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Result<Editor, Error> {
        let mut extensions = HashMap::new();
        extensions.insert("rs".to_string(), "rust".to_string());
        extensions.insert("toml".to_string(), "toml".to_string());
        extensions.insert("txt".to_string(), "text".to_string());
        extensions.insert("".to_string(), "unknown".to_string());

        let file_extension = buffer.file_extension().unwrap_or("").to_string();

        let mut highlighters = HashMap::new();
        if let "rs" = file_extension.as_str() {
            highlighters.insert(
                FileExtension::Rust,
                Box::<RustHighlighter>::default() as Box<dyn Highlighter>,
            );
        }

        Ok(Editor {
            terminal: Terminal::new()?,
            offset_y: 0,
            cursor_position: Position::default(),
            current_line_length: buffer.get_line_length(0),
            status: String::new(),
            command: String::new(),
            mode: EditorMode::Normal,
            retain_command_text: false,
            running: true,
            untouched: buffer.file_path().is_none(),
            debug_bar: false,
            buffer,
            highlighters,
            extensions,
            file_extension,
            search_mode: SearchMode::None,
            search_occurences: vec![],
            search_occurence_idx: 0,
            search_cursor_position: Position::default(),
            search_offset_y: 0,
        })
    }

    pub fn load_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
        self.current_line_length = self.buffer.get_line_length(0);
    }

    fn adjusted_cursor_position(&self) -> Position {
        Position::new(
            std::cmp::min(self.cursor_position.x, self.current_line_length),
            self.cursor_position.y,
        )
    }

    pub fn run(&mut self) {
        let stdin = stdin();
        while self.running {
            self.terminal.clear();

            self.draw_buffer();
            self.draw_status_bar();
            self.draw_command();
            self.draw_debug();

            // Position cursor
            self.terminal.goto(&self.adjusted_cursor_position());
            self.terminal.flush();

            self.handle_user_input(&stdin);
        }
    }

    fn handle_user_input(&mut self, stdin: &Stdin) {
        let key = self.terminal.read_key(stdin).unwrap();
        self.untouched = false;

        match key {
            Key::Char(c) => {
                if c == '\n' {
                    match self.mode {
                        EditorMode::Normal => self.move_down(),
                        EditorMode::Insert => {
                            self.reset_cursor();
                            self.buffer.insert_new_line(&Position::new(
                                self.cursor_position.x,
                                self.offset_y + self.cursor_position.y,
                            ));
                            self.cursor_position.x = 0;
                            self.cursor_position.y += 1;
                        }
                        EditorMode::Command => self.run_command().unwrap_or(()),
                    }
                } else if c == ':' && self.mode == EditorMode::Normal {
                    self.change_mode(EditorMode::Command);
                    self.command.push(':');
                } else {
                    match self.mode {
                        EditorMode::Insert => {
                            self.reset_cursor();
                            self.buffer.insert(
                                c.to_string().as_str(),
                                &Position::new(
                                    self.cursor_position.x,
                                    self.cursor_position.y + self.offset_y,
                                ),
                            );
                            self.cursor_position.x += 1;
                        }
                        EditorMode::Command => {
                            self.command.push(c);

                            if self.command.starts_with('/') && self.command.len() > 1 {
                                self.search_occurences =
                                    self.buffer.find(&self.command.as_str()[1..], 0, true);
                                self.search_mode = SearchMode::Incremental;
                            } else if c == '/' && self.command.is_empty() {
                                self.search_offset_y = self.offset_y;
                            }
                        }
                        _ => self.handle_key_normal_mode(c),
                    }
                }
                self.current_line_length = self
                    .buffer
                    .get_line_length(self.offset_y + self.cursor_position.y);
            }
            Key::Esc => {
                if self.mode == EditorMode::Insert || self.mode == EditorMode::Command {
                    self.search_mode = SearchMode::None;
                    self.search_offset_y = 0;
                    self.change_mode(EditorMode::Normal);
                }
            }
            Key::Ctrl(c) => {
                if c == 'q' {
                    self.quit();
                }
                if c == 'w' {
                    self.save_buffer().unwrap();
                }
            }
            Key::Backspace => {
                if self.mode == EditorMode::Command && !self.command.is_empty() {
                    self.command.pop();

                    if self.command.is_empty() {
                        self.change_mode(EditorMode::Normal);
                        self.search_occurences.clear();
                    } else if self.command.starts_with('/') && self.command.len() > 1 {
                        self.search_occurences =
                            self.buffer.find(&self.command.as_str()[1..], 0, false);
                    }
                    return;
                }

                self.reset_cursor();

                if self.mode == EditorMode::Insert {
                    if self.cursor_position.x > 0 {
                        self.cursor_position.x -= 1;
                        self.buffer.delete(
                            &Position::new(
                                self.cursor_position.x,
                                self.cursor_position.y + self.offset_y,
                            ),
                            1,
                        );
                    } else if self.cursor_position.y > 0 {
                        let line_len = self
                            .buffer
                            .get_line_length(self.offset_y + self.cursor_position.y - 1);
                        self.cursor_position.x = line_len;
                        self.cursor_position.y -= 1;
                        self.buffer.delete(
                            &Position::new(
                                self.cursor_position.x,
                                self.cursor_position.y + self.offset_y,
                            ),
                            2,
                        );
                        self.current_line_length = line_len;
                    } else {
                        // empty
                        return;
                    }

                    self.current_line_length = self
                        .buffer
                        .get_line_length(self.offset_y + self.cursor_position.y);
                } else if self.mode == EditorMode::Normal {
                    if self.cursor_position.x > 0 {
                        self.cursor_position.x -= 1;
                    } else if self.cursor_position.y > 0 {
                        let line_len = self
                            .buffer
                            .get_line_length(self.offset_y + self.cursor_position.y - 1);
                        self.cursor_position.x = line_len;
                        self.cursor_position.y -= 1;
                        self.current_line_length = line_len;
                    }
                }
            }
            Key::Delete => {
                if self.mode == EditorMode::Insert {
                    if self.cursor_position.x < self.current_line_length {
                        self.current_line_length -= 1;
                        self.buffer.delete(
                            &Position::new(
                                self.cursor_position.x,
                                self.cursor_position.y + self.offset_y,
                            ),
                            1,
                        );
                    } else if self.cursor_position.x == self.current_line_length
                        && self.cursor_position.x > 0
                    {
                        self.current_line_length -= 1;
                        self.cursor_position.x = self.current_line_length;
                        self.buffer.delete(
                            &Position::new(
                                self.cursor_position.x,
                                self.cursor_position.y + self.offset_y,
                            ),
                            1,
                        );
                    }
                }
            }
            Key::Left => self.move_left(),
            Key::Right => self.move_right(),
            Key::Up => self.move_up(),
            Key::Down => self.move_down(),
            Key::Home => self.move_to_sol(),
            Key::End => self.move_to_eol(),
            Key::PageUp => self.move_page_up(),
            Key::PageDown => self.move_page_down(),
            _ => {
                dbg!(&key);
            }
        }
    }

    fn run_command(&mut self) -> std::io::Result<()> {
        if self.command.starts_with('/') {
            if !self.search_occurences.is_empty() {
                self.cursor_position = self.search_cursor_position;
            }
            self.search_mode = SearchMode::None;
            self.offset_y = self.search_offset_y;
            self.search_offset_y = 0;
            self.change_mode(EditorMode::Normal);
            return Ok(());
        }

        let tokens: Vec<&str> = self.command.split(':').collect();
        if tokens.len() <= 1 {
            return Ok(());
        }

        let pre_command = tokens[0];
        let command = tokens[1];

        match command {
            "q" => self.quit(),
            "w" => self.save_buffer()?,
            "wq" => {
                self.save_buffer()?;
                self.quit();
            }
            "help" => self.print_help(),
            "debug" => self.toggle_debug_bar(),
            _ => {
                if pre_command.contains("-- Create file") {
                    let path = format!("{}/{}", env::current_dir()?.display(), tokens[1]);
                    self.buffer.set_file_path(path);
                    self.clear_command();
                    self.save_buffer()?;
                } else {
                    self.command = "Command not found!".to_string();
                    self.retain_command_text = true;
                    self.change_mode(EditorMode::Normal);
                }
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.terminal.clear();
        self.running = false;
    }

    fn save_buffer(&mut self) -> std::io::Result<()> {
        if let Ok(file_path) = self.buffer.save_file() {
            self.command = format!("-- File saved to {}.", file_path);
        } else {
            self.command = "-- Create file:".to_string();
            self.change_mode(EditorMode::Command);
        }
        Ok(())
    }

    fn handle_key_normal_mode(&mut self, key: char) {
        match key {
            'i' => self.change_mode(EditorMode::Insert),
            'k' => self.move_up(),
            'j' => self.move_down(),
            'h' => self.move_left(),
            'l' => self.move_right(),
            '0' => self.move_to_sol(),
            '$' => self.move_to_eol(),
            'A' => {
                self.move_to_eol();
                self.change_mode(EditorMode::Insert);
            }
            '/' => {
                self.change_mode(EditorMode::Command);
                self.command.push('/');
            }
            'n' => self.search_next(),
            'N' => self.search_previous(),
            _ => {}
        }
    }

    fn clear_command(&mut self) {
        if !self.retain_command_text {
            self.command.clear();
            self.retain_command_text = false;
        }
    }

    fn change_mode(&mut self, mode: EditorMode) {
        match mode {
            EditorMode::Insert => {
                self.command = "-- INSERT --".to_string();
            }
            EditorMode::Normal => {
                if self.mode == EditorMode::Command {
                    self.clear_command();
                } else if self.mode == EditorMode::Insert {
                    self.clear_command();
                }
            }
            EditorMode::Command => {
                self.clear_command();
            }
        }
        self.mode = mode;
    }

    fn move_up(&mut self) {
        if self.cursor_position.y == 0 && self.offset_y > 0 {
            self.offset_y -= 1;
        } else if self.cursor_position.y > 0 {
            self.cursor_position.y -= 1;
        }
        self.current_line_length = self
            .buffer
            .get_line_length(self.offset_y + self.cursor_position.y);
    }

    fn move_down(&mut self) {
        let is_valid_line = self.is_valid_line(self.offset_y + self.cursor_position.y + 1);
        if is_valid_line && self.cursor_position.y == self.draw_terminal_size().1 {
            self.offset_y += 1;
        } else if is_valid_line {
            self.cursor_position.y += 1;
        }
        self.current_line_length = self
            .buffer
            .get_line_length(self.offset_y + self.cursor_position.y);
    }

    fn move_right(&mut self) {
        self.reset_cursor();
        let new_position = Position {
            x: self.cursor_position.x + 1,
            y: self.offset_y + self.cursor_position.y,
        };
        if self.is_valid_column(&new_position) {
            self.cursor_position.x += 1;
        }
    }

    fn move_left(&mut self) {
        self.reset_cursor();
        if self.cursor_position.x > 0 {
            self.cursor_position.x -= 1;
        }
    }

    fn move_page_up(&mut self) {
        let height = self.terminal.size().1 - 3;
        if self.offset_y > height as usize {
            self.offset_y -= height as usize;
        } else {
            self.offset_y = 0;
        }

        self.current_line_length = self
            .buffer
            .get_line_length(self.offset_y + self.cursor_position.y);
    }

    fn move_page_down(&mut self) {
        let total_lines = self.buffer.get_total_lines();
        let height = self.terminal.size().1 - 3;
        if (self.offset_y + height as usize) < total_lines {
            self.offset_y += height as usize;
        }

        self.current_line_length = self
            .buffer
            .get_line_length(self.offset_y + self.cursor_position.y);
    }

    /// Moves the cursor to start of line.
    fn move_to_sol(&mut self) {
        self.cursor_position.x = 0;
    }

    /// Moves the cursor to end of line.
    fn move_to_eol(&mut self) {
        self.cursor_position.x = self.current_line_length;
    }

    fn reset_cursor(&mut self) {
        if self.cursor_position.x > self.current_line_length {
            self.cursor_position.x = self.current_line_length;
        }
    }

    /// Check if the buffer contains the line. Use 0-based alignment.
    fn is_valid_line(&self, line: usize) -> bool {
        line < self.buffer.get_total_lines()
    }

    /// Check if the buffer contains the column for the line. Use 0-based alignment.
    /// Allow 1 space after last character.
    fn is_valid_column(&self, position: &Position) -> bool {
        if position.x > self.current_line_length {
            return false;
        }

        if position.y + 1 == self.buffer.get_total_lines()
            && position.x == 0
            && self.current_line_length == 0
        {
            return true;
        }

        if self.current_line_length == 0 {
            return false;
        }

        true
    }

    fn draw_buffer(&mut self) {
        self.terminal.hide_cursor();
        self.terminal.goto(&Position::default());

        let offset_y = match self.search_mode {
            SearchMode::None => self.offset_y,
            SearchMode::Incremental => self.search_offset_y,
        };

        let buffer = &self.buffer.get(
            &Position::new(0, offset_y),
            Some(&Position::new(
                0,
                offset_y + self.draw_terminal_size().1 + 1,
            )),
        );

        if self.untouched {
            let text = "Nimbus Text Editor";
            let version = "Version 0.1.0";
            let help = "Type :help for usage manual.";

            let (w, h) = self.terminal.size();
            let mut pos = Position::new(
                w.saturating_div(2) as usize - (text.len() / 2),
                h.saturating_div(2) as usize,
            );

            self.terminal.goto(&pos);
            self.terminal.write_with_color(text, &color::Yellow);

            pos.x = w.saturating_div(2) as usize - (version.len() / 2);
            pos.y += 1;
            self.terminal.goto(&pos);
            self.terminal.write(version);

            pos.x = w.saturating_div(2) as usize - (help.len() / 2);
            pos.y += 2;
            self.terminal.goto(&pos);
            self.terminal.write_with_color(help, &color::White);
        } else if let Some(highlighter) = self.highlighters.get(&FileExtension::Rust) {
            highlighter.highlight(buffer, &self.terminal);
        } else {
            self.terminal.write(buffer);
        }

        if self.search_mode == SearchMode::Incremental && !self.search_occurences.is_empty() {
            let occurences = vec![self.search_occurences[self.search_occurence_idx].clone()];
            let offset = self
                .buffer
                .get_offset_from_position(&Position::new(0, offset_y))
                .unwrap_or(0);

            for range in occurences.iter() {
                let mut pos = self.buffer.get_position_from_offset(range.start);
                
                if pos.y < offset_y || pos.y > offset_y + self.draw_terminal_size().1 {
                    self.search_offset_y = pos.y.saturating_sub(self.draw_terminal_size().1);
                    return;
                }

                let start = range.start - offset;
                let end = range.end - offset;

                pos.y = pos.y.saturating_sub(offset_y);
                self.search_cursor_position = pos;
                self.terminal.goto(&pos);

                if self.mode == EditorMode::Command {
                    self.terminal.write_with_color_bg(
                        &buffer[start..end],
                        &color::Black,
                        &color::LightYellow,
                    );
                } else {
                    self.terminal
                        .write_with_color(&buffer[start..end], &color::Reset);
                }    
            }
        }

        self.terminal.show_cursor();
    }

    fn draw_command(&mut self) {
        self.terminal.goto(&Position {
            x: 0,
            y: self.terminal.size().1 as usize - 1,
        });
        self.terminal.write(&self.command);
    }

    fn draw_status_bar(&mut self) {
        if self.untouched {
            return;
        }

        let left_side = self.buffer.file_path().unwrap_or("[No Name]").to_string();
        let right_side = format!(
            "{} | {}:{}",
            self.get_extension_name(&self.file_extension),
            self.cursor_position.y,
            self.cursor_position.x,
        );
        let pad_left = self.terminal.size().0 as usize - left_side.len() - 2;
        self.status = format!(" {}{:>width$} ", left_side, right_side, width = pad_left);

        self.terminal.goto(&Position {
            x: 0,
            y: self.terminal.size().1 as usize - 2,
        });
        self.terminal
            .write_with_color_bg(&self.status, &color::Black, &color::Magenta);
    }

    fn draw_debug(&mut self) {
        if self.untouched || !self.debug_bar {
            return;
        }

        // Debug bar
        // let debug = self
        //     .buffer
        //     .get_debug_status(&self.adjusted_cursor_position());
        let debug = format!(
            "offset_y={} | Search: offset_y={}, pos=({},{}).", self.offset_y,
            self.search_offset_y, self.search_cursor_position.x, self.search_cursor_position.y
        );

        self.terminal.goto(&Position {
            x: 0,
            y: self.terminal.size().1 as usize,
        });
        self.terminal.write(&debug);
    }

    /// Print the keybind information into the command bar.
    fn print_help(&mut self) {
        self.command = "<C-q> - Exit, <C-w> - Save | Command: :q - quit, :w - write, :debug - toggle debug bar".to_string();
    }

    fn toggle_debug_bar(&mut self) {
        self.debug_bar = !self.debug_bar;
        self.clear_command();
    }

    fn get_extension_name(&self, extension: &str) -> &str {
        if let Some(name) = self.extensions.get(extension) {
            return name;
        }
        self.get_extension_name("")
    }

    fn search_next(&mut self) {
        if self.search_occurence_idx < self.search_occurences.len() - 1 {
            self.search_occurence_idx += 1;
        } else {
            self.search_occurence_idx = 0;
        }

        let range = &self.search_occurences[self.search_occurence_idx];
        let pos = self.buffer.get_position_from_offset(range.start);
        if pos.y < self.offset_y || pos.y > self.offset_y + self.draw_terminal_size().1 {
            self.offset_y = pos.y.saturating_sub(self.draw_terminal_size().1);
        }
        self.cursor_position = Position::new(pos.x, pos.y.saturating_sub(self.offset_y));
    }

    fn search_previous(&mut self) {
        if self.search_occurence_idx > 0 {
            self.search_occurence_idx -= 1;
        } else {
            self.search_occurence_idx = self.search_occurences.len() - 1;
        }
        let range = &self.search_occurences[self.search_occurence_idx];
        let pos = self.buffer.get_position_from_offset(range.start);
        if pos.y < self.offset_y || pos.y > self.offset_y + self.draw_terminal_size().1 {
            self.offset_y = pos.y;
        }
        self.cursor_position = Position::new(pos.x, pos.y.saturating_sub(self.offset_y));
    }

    fn draw_terminal_size(&self) -> (usize, usize) {
        let (w, h) = self.terminal.size();
        (w.into(), (h - 4).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_column() {
        let buffer = Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.".to_string());
        let mut editor = Editor::new(buffer).unwrap();

        assert!(editor.is_valid_column(&Position::new(0, 0)));
        assert!(editor.is_valid_column(&Position::new(13, 0)));

        editor.move_down();
        assert!(editor.is_valid_column(&Position::new(0, 1)));
        assert!(editor.is_valid_column(&Position::new(14, 1)));

        editor.move_down();
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

        let buffer =
            Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.\r\n".to_string());
        let editor = Editor::new(buffer).unwrap();

        assert!(editor.is_valid_line(3));
        assert!(!editor.is_valid_line(4));
    }

    #[test]
    fn test_editor_complex_1() {
        let buffer =
            Buffer::from_string("File is read.\r\nThe hero lied.\r\nThe end.\r\n".to_string());
        let mut editor = Editor::new(buffer).unwrap();

        editor.buffer.delete(&Position::new(14, 1), 2);
        assert_eq!(
            editor.buffer.get(&Position::new(0, 0), None),
            "File is read.\r\nThe hero lied.The end.\r\n"
        );

        assert_eq!(editor.buffer.get_line_length(1), 22);
        assert_eq!(editor.buffer.get_total_lines(), 3);

        editor.move_down();
        assert!(editor.is_valid_column(&Position { x: 22, y: 1 }));
        assert!(!editor.is_valid_column(&Position { x: 23, y: 1 }));
        assert!(editor.is_valid_line(1));
        assert!(editor.is_valid_line(2));
        assert!(!editor.is_valid_line(3));
    }
}
