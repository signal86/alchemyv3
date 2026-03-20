use super::semantic::*;
use super::syntax::*;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenType;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
// use std::io;

// kind == "warning" || "error"
macro_rules! print_issues {
    ($vec:expr, $kind:expr) => {
        if !$vec.is_empty() {
            for i in $vec.iter() {
                println!("{} on line {}: {}", $kind, i.0, i.1);
            }
        }
    };
}

// kind == "warning" || "error"
macro_rules! return_issues {
    ($vec:expr, $kind:expr) => {
        print_issues!($vec, $kind);
        if !$vec.is_empty() {
            return None;
        }
    };
}

pub fn expect_token(lexer: &Lexer, t: TokenType) -> Result<(), Error> {
    match lexer.curr_token.t {
        comp if comp == t => Ok(()),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("expected token type {:?}", t),
        )),
    }
}

pub fn expect(lexer: &Lexer, t: TokenType, lexeme: &str) -> Result<(), Error> {
    expect_token(lexer, t)?;
    match lexer.curr_token.lexeme.as_str() {
        comp if comp == lexeme => Ok(()),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("expected value {}", lexeme),
        )),
    }
}

pub fn parse_file(file: File) -> Option<AST> {
    let mut ast = AST { nodes: Vec::new() };
    let mut errors: Vec<(u128, Error)> = Vec::new();
    let mut warnings: Vec<(u128, String)> = Vec::new();

    let mut lexer = Lexer::new(file);
    // ast.nodes.push(parse_Node(&mut lexer));
    lexer.consume_token();
    // lexer.curr_token.t = TokenType::INVALID;

    // Syntactical Analysis and AST generation
    while lexer.curr_token.t != TokenType::EOF {
        // for _ in 0..2 {
        let node = parse_Node(&mut lexer);
        println!("{:#?}", node);
        match node {
            Ok(s) => {
                ast.nodes.push(s);
            }
            Err(e) => {
                errors.push((lexer.line - 1, e));
                lexer.consume_token();
            }
        }
    }

    return_issues!(errors, "error");

    // Semantic Analysis
    match semantic_analysis(&ast) {
        Some(issues) => {
            for (line, issue) in issues.into_iter() {
                match issue {
                    Issue::Error(e) => errors.push((line, e)),
                    Issue::Warn(s) => warnings.push((line, s.to_string())),
                }
            }
        }
        None => {}
    }

    print_issues!(warnings, "warning");
    return_issues!(errors, "error");

    Some(ast)
}
