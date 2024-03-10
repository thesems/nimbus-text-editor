#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Constant,
    Identifier,
    Comment,
}

pub trait Tokenizer<'a> {
    fn new(text: &'a str) -> Self;
    fn next(&mut self) -> Option<TokenType>;
    fn peek(&self) -> Option<TokenType>;
    fn token_type(&self) -> Option<&TokenType>;
    fn token(&self) -> &str;
}

pub struct RustTokenizer<'a> {
    text: &'a str,
    token: &'a str,
    token_type: Option<TokenType>,
    counter: usize,
    keywords: Vec<String>,
    symbols: Vec<char>,
}
impl<'a> Tokenizer<'a> for RustTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            token: "",
            token_type: None,
            counter: 0,
            keywords: vec![
                "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
                "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
                "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
                "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
            ]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
            symbols: vec![
                '!', '=', '%', '&', '*', '+', '!', '-', '.', ',', '>', '<', '/', ':', ';', '@',
                '|', '?', '\"', '\'', '^', '$', '#', '{', '}', '[', ']', '(', ')',
            ],
        }
    }

    fn next(&mut self) -> Option<TokenType> {
        self.token = "";
        let mut token_type = None;
        let mut string_constant = false;
        let mut comment = false;

        for (i, ch) in self.text.chars().skip(self.counter).enumerate() {
            let comp = &self.text[self.counter..self.counter + i + 1];

            if comment {
                if ch == '\n' {
                    token_type = Some(TokenType::Comment);
                    self.token = comp;
                    self.counter += i + 1;
                    break;
                }
                continue;
            }

            let next_ch = self.text.chars().nth(self.counter + i + 1);
            if ch == '/' && next_ch.is_some() && next_ch.unwrap() == '/' {
                comment = true;
                continue;
            }

            if string_constant {
                if ch == '"' {
                    token_type = Some(TokenType::Constant);
                    self.token = comp;
                    self.counter += i + 1;
                    break;
                }
                continue;
            }

            if comp.trim().is_empty() {
                continue;
            }

            let mut keyword_end = false;
            let next_ch = self.text.chars().nth(self.counter + i + 1);
            if next_ch.is_some() {
                keyword_end = !next_ch.unwrap().is_alphanumeric() && next_ch.unwrap() != '_';
            }

            if keyword_end && self.keywords.contains(&comp.trim().to_string()) {
                token_type = Some(TokenType::Keyword);
                self.token = comp;
                self.counter += i + 1;
                break;
            }

            if ch == '"' && !string_constant {
                string_constant = true;
                continue;
            }

            if self.symbols.contains(&ch) {
                token_type = Some(TokenType::Symbol);
                self.token = comp;
                self.counter += i + 1;
                break;
            }

            if comp.trim().parse::<usize>().is_ok() {
                token_type = Some(TokenType::Constant);
                self.token = comp;
                self.counter += i + 1;
                break;
            }

            if let Some(next_ch) = next_ch {
                if next_ch == ' ' || self.symbols.contains(&next_ch) {
                    self.token = comp;
                    self.counter += i + 1;
                    token_type = Some(TokenType::Identifier);
                    break;
                }
            }
        }

        token_type
    }

    fn peek(&self) -> Option<TokenType> {
        todo!()
    }

    fn token_type(&self) -> Option<&TokenType> {
        self.token_type.as_ref()
    }

    fn token(&self) -> &str {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_rust() {
        let code = "use nimbus_text_editor::{buffer::Buffer, editor::Editor};fn main() -> Result<(), Error> { let args: Vec<String> = env::args().skip(1).collect(); Ok(()) }";

        let mut tokenizer = RustTokenizer::new(code);
        assert_eq!(tokenizer.next().unwrap(), TokenType::Keyword);
        assert_eq!(tokenizer.token(), "use");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "nimbus_text_editor");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "{");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "buffer");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "Buffer");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ",");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "editor");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "Editor");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "}");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ";");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Keyword);
        assert_eq!(tokenizer.token(), "fn");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "main");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token().trim(), "-");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ">");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "Result");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "<");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ",");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "Error");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ">");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token().trim(), "{");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Keyword);
        assert_eq!(tokenizer.token().trim(), "let");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "args");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "Vec");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "<");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "String");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ">");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token().trim(), "=");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "env");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ":");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "args");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ".");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "skip");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Constant);
        assert_eq!(tokenizer.token(), "1");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ".");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token(), "collect");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ";");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Identifier);
        assert_eq!(tokenizer.token().trim(), "Ok");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), "(");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token(), ")");
        assert_eq!(tokenizer.next().unwrap(), TokenType::Symbol);
        assert_eq!(tokenizer.token().trim(), "}");
        assert_eq!(tokenizer.next(), None);
        assert_eq!(tokenizer.token(), "");
    }
}
