use crate::{terminal::Terminal, tokenizer::{RustTokenizer, TokenType, Tokenizer}};

pub trait Highlighter {
    fn highlight(&self, content: &str, terminal: &Terminal);
}

pub struct RustHighlighter {
    keyword_color: &'static dyn termion::color::Color,
    constant_color: &'static dyn termion::color::Color,
    identifier_color: &'static dyn termion::color::Color,
    type_color: &'static dyn termion::color::Color,
    symbol_color: &'static dyn termion::color::Color,
}

impl RustHighlighter {
    fn new() -> RustHighlighter {
        RustHighlighter {
            keyword_color: &termion::color::LightRed,
            constant_color: &termion::color::LightGreen,
            identifier_color: &termion::color::LightWhite,
            type_color: &termion::color::LightYellow,
            symbol_color: &termion::color::LightWhite,
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
        let mut tokenizer = RustTokenizer::new(content);

        while let Some(token_type) = tokenizer.next() {
            let color = match token_type {
                TokenType::Keyword => self.keyword_color,
                TokenType::Constant => self.constant_color,
                TokenType::Identifier => {
                    if tokenizer.token().chars().next().unwrap().is_uppercase() {
                        self.type_color
                    } else {
                        self.identifier_color
                    }
                }
                TokenType::Symbol => self.symbol_color,
            };
          
            terminal.write_with_color(tokenizer.token(), color);
        }
    }
}
