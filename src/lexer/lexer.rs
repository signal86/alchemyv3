use super::token::Token;
use super::token::TokenType;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::io;

fn lex_keyword(lexeme: &str) -> bool {
    let keywords = ["create", "meta", "component", "components", "view"];
    if keywords.contains(&lexeme) {
        return true;
    }
    false
}

// [a-zA-Z_]+[a-zA-Z0-9_]*
fn lex_identifier(lexeme: &str) -> bool {
    let pattern = r"^[a-zA-Z_]+[a-zA-Z0-9_]*$";
    let re = Regex::new(pattern).unwrap();
    if re.is_match(lexeme) {
        return true;
    }
    false
}

// [0-9]+(\.[0-9]+)?
fn lex_number(lexeme: &str) -> bool {
    let pattern = r"^[0-9]+(\.[0-9]+)?$";
    let re = Regex::new(pattern).unwrap();
    if re.is_match(lexeme) {
        return true;
    }
    false
}

// "(\\.|[^"])*"
fn lex_string(lexeme: &str) -> bool {
    let pattern = r#"^"(\\.|[^"])*"$"#;
    let re = Regex::new(pattern).unwrap();
    if re.is_match(lexeme) {
        return true;
    }
    false
}

fn lex_operator(lexeme: &str) -> bool {
    match lexeme {
        "=" => true,
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        "." => true,
        ":" => true,
        _ => false,
    }
}

fn match_buffer(buffer: &mut String, tokens: &mut Vec<Token>) {
    if !buffer.is_empty() {
        let mut b = buffer.clone();
        tokens.push(Token {
            t: match &b {
                string if lex_keyword(string) => TokenType::Keyword,
                string if lex_string(string) => {
                    b.pop();
                    b.remove(0);
                    TokenType::String
                }
                string if lex_number(string) => TokenType::Number,
                string if lex_operator(string) => TokenType::Operator,
                string if lex_identifier(string) => TokenType::Identifier,
                string if string == ";" => TokenType::Terminator,
                string if string == "{" => TokenType::OpenBrace,
                string if string == "}" => TokenType::CloseBrace,
                string if string == "[" => TokenType::OpenBracket,
                string if string == "]" => TokenType::CloseBracket,
                string if string == "," => TokenType::Separator,
                _ => TokenType::INVALID,
            },
            lexeme: b,
        });
        buffer.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub curr_token: Token,
    // pub reader: &BufReader<&File>,
}

impl Lexer {
    pub fn consume_token(&self) {
        // next_token(self.reader);
        // TEMP
        next_token("");
    }
}

// TODO: make work
pub fn next_token(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut buffer = String::new();

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;

        if c.is_whitespace() {
            match_buffer(&mut buffer, &mut tokens);
            continue;
        }

        if c == '/' && i < chars.len() && chars[i] == '/' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // only works with 1 character operators
        if lex_operator(&c.to_string()) {
            match_buffer(&mut buffer, &mut tokens);
            tokens.push(Token {
                t: TokenType::Operator,
                lexeme: c.to_string(),
            });
            continue;
        }

        if c == ',' {
            match_buffer(&mut buffer, &mut tokens);
            tokens.push(Token {
                t: TokenType::Separator,
                lexeme: c.to_string(),
            });
            continue;
        }

        if c == ';' {
            match_buffer(&mut buffer, &mut tokens);
            tokens.push(Token {
                t: TokenType::Terminator,
                lexeme: c.to_string(),
            });
            continue;
        }

        if c == '"' {
            match_buffer(&mut buffer, &mut tokens);
            let mut j = i;
            buffer.push(c);
            while j < chars.len() {
                buffer.push(chars[j]);
                if chars[j] == '"' && chars[j - 1] != '\\' {
                    break;
                }
                j += 1;
            }
            match_buffer(&mut buffer, &mut tokens);
            i = j + 1;
            continue;
        }

        buffer.push(c);
    }

    match_buffer(&mut buffer, &mut tokens);
    tokens
}
