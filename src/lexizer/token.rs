#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    String,
    Number,
    Operator,

    INVALID,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub lexeme: Option<String>,
    pub line: u128,
}
