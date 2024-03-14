use std::ops::Range;

use termion::color::Bg;

use crate::{terminal::Terminal, tokenizer::{RustTokenizer, TokenType, Tokenizer}};

pub trait Highlighter {
    fn highlight(&self, content: &str, terminal: &Terminal, search_occurences: Vec<Range<usize>>);
}

pub struct RustHighlighter {
    keyword_color: &'static dyn termion::color::Color,
    constant_color: &'static dyn termion::color::Color,
    identifier_color: &'static dyn termion::color::Color,
    type_color: &'static dyn termion::color::Color,
    symbol_color: &'static dyn termion::color::Color,
    comment_color: &'static dyn termion::color::Color,
    search_color: &'static dyn termion::color::Color,
    search_bg_color: &'static dyn termion::color::Color,
}

impl RustHighlighter {
    fn new() -> RustHighlighter {
        RustHighlighter {
            keyword_color: &termion::color::LightRed,
            constant_color: &termion::color::LightGreen,
            identifier_color: &termion::color::LightWhite,
            type_color: &termion::color::LightYellow,
            symbol_color: &termion::color::LightWhite,
            comment_color: &termion::color::White,
            search_color: &termion::color::Black,
            search_bg_color: &termion::color::LightYellow,
        }
    }
}

impl Default for RustHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for RustHighlighter {
    fn highlight(&self, content: &str, terminal: &Terminal, search_occurences: Vec<Range<usize>>) {
        let mut tokenizer = RustTokenizer::new(content, search_occurences);

        while let Some(token_type) = tokenizer.next() {
            let mut bg_color: &'static dyn termion::color::Color = &termion::color::Reset;
            let color = match token_type {
                TokenType::Keyword => self.keyword_color,
                TokenType::Constant => self.constant_color,
                TokenType::Identifier => {
                    if tokenizer.token().trim().chars().next().unwrap().is_uppercase() {
                        self.type_color
                    } else {
                        self.identifier_color
                    }
                }
                TokenType::Symbol => self.symbol_color,
                TokenType::Comment => self.comment_color,
                TokenType::Search => {
                    bg_color = self.search_bg_color;
                    self.search_color
                }
            };

            terminal.write_with_color_bg(tokenizer.token(), color, bg_color);
        }
    }
}
