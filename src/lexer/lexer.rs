use super::token::Token;
use super::token::TokenType;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
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

fn match_buffer(buffer: &mut String) -> Option<Token> {
    if !buffer.is_empty() {
        let mut b = buffer.clone();
        let t = Token {
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
        };
        buffer.clear();
        return Some(t);
    }
    buffer.clear();
    None
}

#[derive(Debug)]
pub struct Lexer {
    pub curr_token: Token,
    reader: BufReader<File>,
    pushback: Option<char>,
    eof: bool,
    pub line: u128,
}

impl Lexer {
    pub fn new(file: File) -> Self {
        Lexer {
            curr_token: Token {
                t: TokenType::EOF,
                lexeme: "".to_string(),
            },
            reader: BufReader::new(file),
            pushback: None,
            eof: false,
            line: 0,
        }
    }

    // I made next_token (previously consume_token) a bunch of returns despite having already written the parser as a bunch of reads on curr_token, so this is my stupid fix
    pub fn consume_token(&mut self) {
        self.curr_token = self.next_token();
        println!("consumed {:#?}", self.curr_token);
    }

    // Oops
    fn next_token(&mut self) -> Token {
        // let mut buffer: Vec<char> = Vec::new();
        if self.eof {
            return Token {
                t: TokenType::EOF,
                lexeme: "".to_string(),
            };
        }

        let mut reader_buffer = [0; 1];
        let mut buffer = String::new();
        loop {
            let c: char;
            match self.pushback {
                None => {
                    let read = self.reader.read(&mut reader_buffer).unwrap();
                    if read == 0 {
                        match match_buffer(&mut buffer) {
                            Some(s) => {
                                self.eof = true;
                                return s;
                            }
                            None => {
                                return Token {
                                    t: TokenType::EOF,
                                    lexeme: "".to_string(),
                                }
                            }
                        }
                    }

                    c = reader_buffer[0] as char;
                }
                Some(s) => c = s,
            }

            if self.pushback.is_some() {
                self.pushback = None;
            }

            if c.is_whitespace() {
                match match_buffer(&mut buffer) {
                    Some(s) => return s,
                    None => continue,
                }
            }

            // if c == '/' && i < chars.len() && chars[i] == '/' {
            //     while i < chars.len() && chars[i] != '\n' {
            //         i += 1;
            //     }
            //     continue;
            // }

            if c == '#' {
                loop {
                    let mut void = [0; 1];
                    let read = self.reader.read(&mut void).unwrap();
                    if read == 0 || (void[0] as char) == '\n' {
                        break;
                    }
                }
                continue;
            }

            // only works with 1 character operators
            if lex_operator(&c.to_string()) {
                match match_buffer(&mut buffer) {
                    Some(s) => {
                        self.pushback = Some(c);
                        return s;
                    }
                    None => {
                        return Token {
                            t: TokenType::Operator,
                            lexeme: c.to_string(),
                        }
                    }
                }
            }

            if c == ',' {
                match match_buffer(&mut buffer) {
                    Some(s) => {
                        self.pushback = Some(c);
                        return s;
                    }
                    None => {
                        return Token {
                            t: TokenType::Separator,
                            lexeme: c.to_string(),
                        }
                    }
                }
            }

            if c == ';' {
                match match_buffer(&mut buffer) {
                    Some(s) => {
                        self.pushback = Some(c);
                        return s;
                    }
                    None => {
                        return Token {
                            t: TokenType::Terminator,
                            lexeme: c.to_string(),
                        }
                    }
                }
            }

            // {
            //     let mut void = [0; 1];
            //     let read = self.reader.read(&mut void).unwrap();
            //     if read == 0 || (void[0] as char) == '\n' {
            //         break;
            //     }
            // }

            if c == '"' {
                match match_buffer(&mut buffer) {
                    Some(s) => {
                        self.pushback = Some(c);
                        return s;
                    }
                    // !!! Keep pushback in mind
                    None => {
                        let mut j = c;
                        let mut prev = c;
                        loop {
                            buffer.push(j);
                            let mut str = [0; 1];
                            if let Some(s) = self.pushback {
                                j = s;
                                self.pushback = None
                            } else {
                                let read = self.reader.read(&mut str).unwrap();
                                if read == 0 {
                                    break;
                                }
                                prev = j;
                                j = str[0] as char;
                            }
                            if j == '"' && prev != '\\' {
                                buffer.push(j);
                                break;
                            }
                        }
                        return match_buffer(&mut buffer).unwrap();
                    }
                }
            }

            // if c == '"' {
            //     match_buffer(&mut buffer);
            //     let mut j = i;
            //     buffer.push(c);
            //     while j < chars.len() {
            //         buffer.push(chars[j]);
            //         if chars[j] == '"' && chars[j - 1] != '\\' {
            //             break;
            //         }
            //         j += 1;
            //     }
            //     match_buffer(&mut buffer);
            //     i = j + 1;
            //     continue;
            // }

            buffer.push(c);
        }
    }
}

// TODO: make work
// pub fn next_token(line: &str) -> Vec<Token> {
//     let mut tokens: Vec<Token> = Vec::new();
//     let chars: Vec<char> = line.chars().collect();
//     let mut buffer = String::new();
//
//     let mut i = 0;
//     while i < chars.len() {
//         let c = chars[i];
//         i += 1;
//
//         if c.is_whitespace() {
//             match_buffer(&mut buffer, &mut tokens);
//             continue;
//         }
//
//         if c == '/' && i < chars.len() && chars[i] == '/' {
//             while i < chars.len() && chars[i] != '\n' {
//                 i += 1;
//             }
//             continue;
//         }
//
//         // only works with 1 character operators
//         if lex_operator(&c.to_string()) {
//             match_buffer(&mut buffer, &mut tokens);
//             tokens.push(Token {
//                 t: TokenType::Operator,
//                 lexeme: c.to_string(),
//             });
//             continue;
//         }
//
//         if c == ',' {
//             match_buffer(&mut buffer, &mut tokens);
//             tokens.push(Token {
//                 t: TokenType::Separator,
//                 lexeme: c.to_string(),
//             });
//             continue;
//         }
//
//         if c == ';' {
//             match_buffer(&mut buffer, &mut tokens);
//             tokens.push(Token {
//                 t: TokenType::Terminator,
//                 lexeme: c.to_string(),
//             });
//             continue;
//         }
//
//         if c == '"' {
//             match_buffer(&mut buffer, &mut tokens);
//             let mut j = i;
//             buffer.push(c);
//             while j < chars.len() {
//                 buffer.push(chars[j]);
//                 if chars[j] == '"' && chars[j - 1] != '\\' {
//                     break;
//                 }
//                 j += 1;
//             }
//             match_buffer(&mut buffer, &mut tokens);
//             i = j + 1;
//             continue;
//         }
//
//         buffer.push(c);
//     }
//
//     match_buffer(&mut buffer, &mut tokens);
//     tokens
// }
