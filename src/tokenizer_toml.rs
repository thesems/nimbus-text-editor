use std::collections::VecDeque;

use crate::tokenizer::{TokenType, Tokenizer};

pub struct TokenizerToml<'a> {
    text: &'a str,
    token: &'a str,
    token_type: Option<TokenType>,
    counter: usize,
    symbols: Vec<char>,
    keywords: Vec<String>,
    next_tokens: VecDeque<(&'a str, TokenType)>,
}
impl<'a> Tokenizer<'a> for TokenizerToml<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            token: "",
            token_type: None,
            counter: 0,
            symbols: vec!['[', ']', ',', '=', '{', '}'],
            keywords: vec!["true", "false"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            next_tokens: VecDeque::new(),
        }
    }

    fn next(&mut self) -> Option<TokenType> {
        self.token = "";
        let mut token_type = None;

        if !self.next_tokens.is_empty() {
            let (token, token_type) = self.next_tokens.pop_front().unwrap();
            self.token = token;
            return Some(token_type);
        }

        let mut lines = self.text.split_inclusive("\r\n").skip(self.counter);

        if let Some(line) = lines.next() {
            if line.starts_with('#') {
                token_type = Some(TokenType::Comment);
                self.token = line;
                self.counter += 1;
                return token_type;
            }

            if line.starts_with('[') && line.trim().ends_with(']') {
                token_type = Some(TokenType::Table);
                self.token = line;
                self.counter += 1;
                return token_type;
            }

            // this block should be recursive to support any level of nesting
            let tokens: Vec<&str> = line.split('=').collect();
            if tokens.len() > 1 {
                self.token = tokens[0];

                for token in tokens.iter().skip(1) {
                    self.next_tokens.push_back(("=", TokenType::Symbol));

                    let mut is_string = false;
                    let mut start = 0;
                    let mut i = 0;
                    let mut it = token.chars().peekable();

                    while let Some(ch) = it.next() {
                        let next_ch = *it.peek().unwrap_or(&' ');
                        let comp = match next_ch {
                            '$' => {
                                i += 2;
                                &token[start..i + 1]
                            }
                            _ => &token[start..i + 1],
                        };

                        i += 1;

                        if ch == '\"' {
                            if !is_string {
                                is_string = true;
                            } else {
                                start = i;
                                is_string = false;
                                self.next_tokens
                                    .push_back((comp, TokenType::StringConstant));
                                continue;
                            }
                        }

                        if is_string {
                            continue;
                        }

                        if self.symbols.contains(&ch) {
                            start = i;
                            self.next_tokens.push_back((comp, TokenType::Symbol));
                            continue;
                        }

                        if self.keywords.contains(&comp.trim().to_string()) {
                            start = i;
                            self.next_tokens.push_back((comp, TokenType::Keyword));
                            continue;
                        }

                        if next_ch == ' ' || self.symbols.contains(&next_ch) {
                            start = i;
                            self.next_tokens.push_back((comp, TokenType::Value));
                            continue;
                        }
                    }
                }

                self.counter += 1;
                return Some(TokenType::Key);
            }

            // empty line
            self.token = line;
            self.counter += 1;
            token_type = Some(TokenType::Key);
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
    fn test_tokenizer_toml() {}
}
