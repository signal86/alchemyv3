#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    String,
    Number,
    Operator,
    Separator,
    Terminator, // semicolon

    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    EOF,

    INVALID,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub lexeme: String,
}
