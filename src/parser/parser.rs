use super::super::lexer::token::Token;
use super::super::lexer::token::TokenType;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::io;

/*
CFG

Node ->
    Meta
    | ComponentDefinition
    | ComponentInstance
    | Assignment

Meta ->
    Keyword("meta") MetaSet Terminator(";")

MetaSet ->
    Keyword("components")
    | Keyword("view")

ComponentDefinition ->
    Keyword("create") Keyword("component") Operator(":") Identifier(*) OpenBrace("{") ComponentBody CloseBrace("}") Terminator(";")

ComponentBody ->
    ComponentProperties*

ComponentProperties ->
    VarsP
    | DefaultP
    | HtmlTemplateP
    | CssTemplateP
    | JSTemplateP

VarsP ->
    Identifier("vars") Operator("=") OpenBracket("[") StrList CloseBracket["]"] Terminator(";")

StrList ->
    String(*)
    | String(*) Separator(",") StrList

DefaultP ->
    Identifier("default") Operator("=") String(*) Terminator(";")

HtmlTemplateP ->
    Identifier("html") Operator("=") String(*) Terminator(";")

CssTemplateP ->
    Identifier("css") Operator("=") String(*) Terminator(";")

JSTemplateP ->
    Identifier("js") Operator("=") String(*) Terminator(";")

ComponentInstance ->
    Keyword("create") Identifier(*) Terminator(";")
    | Keyword("create") Identifier(*) Operator(":") Identifier(*) Terminator(";")
    | Keyword("create") Identifier(*) Operator(":") Identifier(*) Operator("=") String(*) Terminator(";")

Assignment ->
    Identifier(*) Operator(".") Identifier(*) Operator("=") String(*) Terminator(";")
*/

#[derive(Debug, Clone)]
pub enum ASTNode {
    Meta(Meta),
    ComponentDefinition(ComponentDefinition),
    ComponentInstance(ComponentInstance),
    Assingment(Assignment),
}

#[derive(Debug, Clone)]
pub enum Meta {
    Components,
    View,
}

#[derive(Debug, Clone)]
pub struct ComponentDefinition {
    pub name: String,
    pub vars: Vec<String>,
    pub default_var: Option<String>,
    pub html: Option<String>, // template
    pub css: Option<String>,  // template too
    pub js: Option<String>,   // template too too
}

#[derive(Debug, Clone)]
pub struct ComponentInstance {
    pub component: String,
    pub identifier: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub component_identifier: String,
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct AST {
    pub nodes: Vec<ASTNode>,
}

#[derive(Debug, Copy, Clone)]
pub enum ParserErrorType {
    Example,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub error_type: ParserErrorType,
    pub msg: String,
    pub line: u128,
}

pub fn parse_file(file: &File) -> Result<AST, Vec<ParserError>> {
    let mut ast = AST { nodes: Vec::new() };
    let mut errors: Vec<ParserError> = Vec::new();

    let r = BufReader::new(file);

    Ok(ast)
}

fn parse_statement(ctx: &[String], statement: Vec<Token>) -> Option<String> {
    for token in statement {
        match token.t {
            TokenType::Keyword => {}
            TokenType::Identifier => {}
            TokenType::String => {}
            TokenType::Number => {}
            TokenType::Separator => {}
            TokenType::Operator => {}
            TokenType::Terminator => {}
            TokenType::OpenBrace => {}
            TokenType::CloseBrace => {}
            TokenType::OpenBracket => {}
            TokenType::CloseBracket => {}
            TokenType::INVALID => {}
        }
    }
    return None;
}
