use crate::{
    highlighter::Highlighter, terminal::Terminal, tokenizer::TokenType, tokenizer::Tokenizer,
    tokenizer_rust::TokenizerRust,
};

pub struct HighlighterRust {
    keyword_color: &'static dyn termion::color::Color,
    constant_color: &'static dyn termion::color::Color,
    identifier_color: &'static dyn termion::color::Color,
    type_color: &'static dyn termion::color::Color,
    symbol_color: &'static dyn termion::color::Color,
    comment_color: &'static dyn termion::color::Color,
}

impl HighlighterRust {
    fn new() -> HighlighterRust {
        HighlighterRust {
            keyword_color: &termion::color::LightRed,
            constant_color: &termion::color::LightGreen,
            identifier_color: &termion::color::LightWhite,
            type_color: &termion::color::LightYellow,
            symbol_color: &termion::color::LightWhite,
            comment_color: &termion::color::White,
        }
    }
}

impl Default for HighlighterRust {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for HighlighterRust {
    fn highlight(&self, content: &str, terminal: &Terminal) {
        let mut tokenizer = TokenizerRust::new(content);

        while let Some(token_type) = tokenizer.next() {
            let color = match token_type {
                TokenType::Keyword => self.keyword_color,
                TokenType::Constant => self.constant_color,
                TokenType::Identifier => {
                    if tokenizer
                        .token()
                        .trim()
                        .chars()
                        .next()
                        .unwrap()
                        .is_uppercase()
                    {
                        self.type_color
                    } else {
                        self.identifier_color
                    }
                }
                TokenType::Symbol => self.symbol_color,
                TokenType::Comment => self.comment_color,
                _ => &termion::color::Reset,
            };

            terminal.write_with_color(tokenizer.token(), color);
        }
    }
}
