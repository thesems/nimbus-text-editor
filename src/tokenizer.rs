#[derive(Debug, PartialEq)]
pub enum TokenType {
    // rust
    Keyword,
    Symbol,
    Constant,
    Identifier,
    Comment, // rust, toml

    // toml
    Table,
    Key,
    Value,
    IntConstant,
    StringConstant,
}

pub trait Tokenizer<'a> {
    fn new(text: &'a str) -> Self;
    fn next(&mut self) -> Option<TokenType>;
    fn peek(&self) -> Option<TokenType>;
    fn token_type(&self) -> Option<&TokenType>;
    fn token(&self) -> &str;
}
