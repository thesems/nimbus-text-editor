use crate::{
    highlighter::Highlighter, terminal::Terminal, tokenizer::TokenType, tokenizer::Tokenizer,
    tokenizer_toml::TokenizerToml,
};

pub struct HighlighterToml {
    table_color: &'static dyn termion::color::Color,
    key_color: &'static dyn termion::color::Color,
    value_color: &'static dyn termion::color::Color,
    string_constant_color: &'static dyn termion::color::Color,
    int_constant_color: &'static dyn termion::color::Color,
    symbol_color: &'static dyn termion::color::Color,
    keyword_color: &'static dyn termion::color::Color,
    comment_color: &'static dyn termion::color::Color,
}

impl HighlighterToml {
    fn new() -> HighlighterToml {
        HighlighterToml {
            table_color: &termion::color::Yellow,
            key_color: &termion::color::LightBlue,
            value_color: &termion::color::LightGreen,
            string_constant_color: &termion::color::Rgb(200, 84, 60),
            int_constant_color: &termion::color::LightGreen,
            symbol_color: &termion::color::Reset,
            keyword_color: &termion::color::Blue,
            comment_color: &termion::color::White,
        }
    }
}

impl Default for HighlighterToml {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for HighlighterToml {
    fn highlight(&self, content: &str, terminal: &Terminal) {
        let mut tokenizer = TokenizerToml::new(content);

        while let Some(token_type) = tokenizer.next() {
            let color = match token_type {
                TokenType::Table => self.table_color,
                TokenType::Key => self.key_color,
                TokenType::Value => self.value_color,
                TokenType::IntConstant => self.int_constant_color,
                TokenType::StringConstant => self.string_constant_color,
                TokenType::Symbol => self.symbol_color,
                TokenType::Keyword => self.keyword_color,
                TokenType::Comment => self.comment_color,
                _ => &termion::color::Reset,
            };

            terminal.write_with_color(tokenizer.token(), color);
        }
    }
}
