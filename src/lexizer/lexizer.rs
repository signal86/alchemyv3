use super::token::Token;
use super::token::TokenType;
use regex::Regex;
// use std::io;

fn lex_keyword(lexeme: &str) -> bool {
    let keywords = ["create", "use"];
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
        "/" => true,
        "." => true,
        ":" => true,
        _ => false,
    }
}

fn match_buffer(buffer: &mut String, tokens: &mut Vec<Token>, line: u128) {
    if !buffer.is_empty() {
        let b = buffer.clone();
        tokens.push(Token {
            t: match &b {
                string if lex_keyword(string) => TokenType::Keyword,
                string if lex_string(string) => TokenType::String,
                string if lex_number(string) => TokenType::Number,
                string if lex_operator(string) => TokenType::Operator,
                string if lex_identifier(string) => TokenType::Identifier,
                _ => TokenType::INVALID,
            },
            lexeme: Some(b),
            line: line,
        });
        buffer.clear();
    }
}

pub fn lex(line: &str, linenum: u128) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut buffer = String::new();

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;

        if c.is_whitespace() {
            match_buffer(&mut buffer, &mut tokens, linenum);
            continue;
        }

        // only works with 1 character operators
        if lex_operator(&c.to_string()) {
            match_buffer(&mut buffer, &mut tokens, linenum);
            tokens.push(Token {
                t: TokenType::Operator,
                lexeme: Some(c.to_string()),
                line: linenum,
            });
            continue;
        }

        if c == '"' {
            match_buffer(&mut buffer, &mut tokens, linenum);
            let mut j = i;
            buffer.push(c);
            while j < chars.len() {
                buffer.push(chars[j]);
                if chars[j] == '"' && chars[j - 1] != '\\' {
                    break;
                }
                j += 1;
            }
            match_buffer(&mut buffer, &mut tokens, linenum);
            i = j + 1;
            continue;
        }

        buffer.push(c);
    }

    match_buffer(&mut buffer, &mut tokens, linenum);
    tokens
}
