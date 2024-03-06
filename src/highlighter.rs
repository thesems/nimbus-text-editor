use crate::terminal::Terminal;

pub trait Highlighter {
    fn highlight(&self, content: &str, terminal: &Terminal);
}

pub struct RustHighlighter {
    keywords: Vec<String>,
    keyword_color: &'static dyn termion::color::Color,
    constant_color: &'static dyn termion::color::Color,
}

impl RustHighlighter {
    fn new() -> RustHighlighter {
        RustHighlighter {
            keywords: vec![
                "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
                "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
                "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
                "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
            ]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
            keyword_color: &termion::color::LightRed,
            constant_color: &termion::color::LightGreen,
        }
    }
}

impl Default for RustHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for RustHighlighter {
    fn highlight(&self, content: &str, terminal: &Terminal) {
        let mut word = String::new();
        let mut constant = false;
        let mut comment = false;

        for ch in content.chars() {
            if word.starts_with("//") && !comment {
                comment = true;
                word.push(ch);
                continue;
            }
            if comment {
                word.push(ch);
                if ch == '\n' {
                    comment = false;
                    terminal.write_with_color(word.as_str(), &termion::color::White);
                    word.clear();
                }
                continue;
            }

            if ch == '"' {
                if constant {
                    word.push(ch);
                    constant = false;
                    terminal.write_with_color(word.as_str(), self.constant_color);
                    word.clear();
                } else {
                    constant = true;
                    terminal.write(word.as_str());
                    word.clear();
                    word.push(ch);
                }
            } else {
                if (ch == ' ' || ch == '\n') && !constant {
                    if self.keywords.contains(&word.to_string()) {
                        terminal.write_with_color(word.as_str(), self.keyword_color);
                    } else if word.parse::<usize>().is_ok() {
                        terminal.write_with_color(word.as_str(), self.constant_color);
                    } else {
                        terminal.write(word.as_str());
                    }
                    terminal.write(ch.to_string().as_str());
                    word.clear();
                    continue;
                }
                word.push(ch);
            }
        }
    }
}
