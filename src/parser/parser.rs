use super::super::lexer::token::Token;
use super::super::lexer::token::TokenType;
// use std::io;

/*
* CFG
*
*/

pub fn parse_statement(ctx: &[String], statement: Vec<Token>) -> Option<String> {
    for token in statement {
        match token.t {
            TokenType::Keyword => {}
            TokenType::Identifier => {}
            TokenType::String => {}
            TokenType::Number => {}
            TokenType::Operator => {}
            TokenType::INVALID => {}
        }
    }
    return None;
}
