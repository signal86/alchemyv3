use super::super::lexer::lexer::Lexer;
use super::super::lexer::token::Token;
use super::super::lexer::token::TokenType;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
// use std::io;

/*
CFG

Node ->
    Meta
    | ComponentDefinition
    | ComponentInstance
    | Assignment

Done
Meta ->
    Keyword("meta") MetaSet Terminator(";")

Done
MetaSet ->
    Keyword("components")
    | Keyword("view")

Done
ComponentDefinition ->
    Keyword("create") Keyword("component") Operator(":") Identifier(*) OpenBrace("{") ComponentBody CloseBrace("}") Terminator(";")

Done
ComponentBody ->
    ComponentProperties*

Done
ComponentProperties ->
    VarsP
    | DefaultP
    | HtmlTemplateP
    | CssTemplateP
    | JSTemplateP

Done
VarsP ->
    Identifier("vars") Operator("=") OpenBracket("[") StrList CloseBracket["]"] Terminator(";")

Done
StrList ->
    String(*)
    | String(*) Separator(",") StrList

Done
DefaultP ->
    Identifier("default") Operator("=") String(*) Terminator(";")

Done
HtmlTemplateP ->
    Identifier("html") Operator("=") String(*) Terminator(";")

Done
CssTemplateP ->
    Identifier("css") Operator("=") String(*) Terminator(";")

Done
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

/*
Meta ->
    Keyword("meta") MetaSet Terminator(";")

MetaSet ->
    Keyword("components")
    | Keyword("view")
*/

#[allow(non_snake_case)]
pub fn parse_MetaSet(lexer: &mut Lexer) -> Result<Meta, Error> {
    expect_token(lexer, TokenType::Keyword)?;
    match lexer.curr_token.lexeme.as_str() {
        "components" => {
            lexer.consume_token();
            Ok(Meta::Components)
        }
        "view" => {
            lexer.consume_token();
            Ok(Meta::View)
        }
        _ => Err(Error::new(ErrorKind::Other, "bad keyword")),
    }

    // if lexer.curr_token.t == TokenType::Keyword && lexer.curr_token.lexeme == "components" {
    //     Ok(Meta::Components)
    // } else if lexer.curr_token.t == TokenType::Keyword && lexer.curr_token.lexeme == "view" {
    //     Ok(Meta::View)
    // } else {
    //     Err(Error::new(ErrorKind::Other, "INVALID"))?
    // }
}

#[allow(non_snake_case)]
pub fn parse_Meta(lexer: &mut Lexer) -> Result<Meta, Error> {
    expect(lexer, TokenType::Keyword, "meta")?;
    lexer.consume_token();
    let s = parse_MetaSet(lexer)?;
    expect_token(lexer, TokenType::Terminator)?;
    Ok(s)

    // match lexer.curr_token.t {
    //     TokenType::Keyword => match lexer.curr_token.lexeme.as_str() {
    //         "meta" => {
    //             lexer.consume_token();
    //             match parse_MetaSet(lexer) {
    //                 Ok(s) => match lexer.curr_token.t {
    //                     TokenType::Terminator => {
    //                         lexer.consume_token();
    //                         Ok(s)
    //                     }
    //                     _ => Err(Error::new(ErrorKind::Other, "terminator not found")),
    //                 },
    //                 Err(E) => Err(E),
    //             }
    //         }
    //         _ => Err(Error::new(ErrorKind::Other, "bad keyword")),
    //     },
    //     _ => Err(Error::new(ErrorKind::Other, "not a keyword")),
    // }
}

/*
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
*/

fn expect_token(lexer: &Lexer, t: TokenType) -> Result<(), Error> {
    match lexer.curr_token.t {
        comp if comp == t => Ok(()),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("expected token type {:?}", t),
        )),
    }
}

fn expect(lexer: &Lexer, t: TokenType, lexeme: &str) -> Result<(), Error> {
    expect_token(lexer, t)?;
    match lexer.curr_token.lexeme.as_str() {
        comp if comp == lexeme => Ok(()),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("expected value {}", lexeme),
        )),
    }
}

#[allow(non_snake_case)]
pub fn parse_StrList(lexer: &mut Lexer) -> Result<Vec<String>, Error> {
    let mut strs: Vec<String> = Vec::new();
    expect_token(lexer, TokenType::String)?;
    strs.push(lexer.curr_token.lexeme.clone());
    lexer.consume_token();
    if lexer.curr_token.t == TokenType::Operator && lexer.curr_token.lexeme == "," {
        lexer.consume_token();
        let mut ext = parse_StrList(lexer)?;
        strs.append(&mut ext);
    }
    Ok(strs)
}

#[allow(non_snake_case)]
pub fn parse_VarsP(lexer: &mut Lexer, def: &mut ComponentDefinition) -> Result<(), Error> {
    expect(lexer, TokenType::Identifier, "vars")?;
    lexer.consume_token();
    expect(lexer, TokenType::Operator, "=")?;
    lexer.consume_token();
    expect_token(lexer, TokenType::OpenBracket)?;
    lexer.consume_token();
    let strs: Vec<String> = parse_StrList(lexer)?;
    expect_token(lexer, TokenType::CloseBracket)?;
    lexer.consume_token();
    expect_token(lexer, TokenType::Terminator)?;
    lexer.consume_token();

    def.vars.extend(strs);

    Ok(())
}

#[allow(non_snake_case)]
fn parse_Template(lexer: &mut Lexer, mutator: &mut Option<String>) -> Result<(), Error> {
    lexer.consume_token();

    expect(lexer, TokenType::Operator, "=")?;
    lexer.consume_token();
    expect_token(lexer, TokenType::String)?;
    *mutator = Some(lexer.curr_token.lexeme.clone());
    lexer.consume_token();
    expect_token(lexer, TokenType::Terminator)?;
    lexer.consume_token();

    Ok(())
}

// #[allow(non_snake_case)]
// fn parse_HtmlTemplateP(lexer: &mut Lexer, def: &mut ComponentDefinition) -> Result<(), Error> {}
//
// #[allow(non_snake_case)]
// fn parse_CssTemplateP(lexer: &mut Lexer, def: &mut ComponentDefinition) -> Result<(), Error> {}
//
// #[allow(non_snake_case)]
// fn parse_JSTemplateP(lexer: &mut Lexer, def: &mut ComponentDefinition) -> Result<(), Error> {}

#[allow(non_snake_case)]
pub fn parse_ComponentProperty(
    lexer: &mut Lexer,
    def: &mut ComponentDefinition,
) -> Result<(), Error> {
    expect_token(lexer, TokenType::Identifier)?;
    match lexer.curr_token.lexeme.as_str() {
        "vars" => parse_VarsP(lexer, def),
        "default" => parse_Template(lexer, &mut def.default_var),
        "html" => parse_Template(lexer, &mut def.html),
        "css" => parse_Template(lexer, &mut def.css),
        "js" => parse_Template(lexer, &mut def.js),
        _ => Err(Error::new(ErrorKind::Other, "invalid property")),
    }
}

#[allow(non_snake_case)]
pub fn parse_ComponentBody(lexer: &mut Lexer, n: String) -> Result<ComponentDefinition, Error> {
    let mut def = ComponentDefinition {
        name: n,
        vars: Vec::new(),
        default_var: None,
        html: None,
        css: None,
        js: None,
    };

    while lexer.curr_token.t != TokenType::CloseBrace {
        parse_ComponentProperty(lexer, &mut def)?;
    }

    Ok(def)
}

#[allow(non_snake_case)]
fn parse_ComponentDefinition(lexer: &mut Lexer) -> Result<ComponentDefinition, Error> {
    expect(lexer, TokenType::Keyword, "create")?;
    lexer.consume_token();
    expect(lexer, TokenType::Keyword, "component")?;
    lexer.consume_token();
    expect(lexer, TokenType::Operator, ":")?;
    lexer.consume_token();
    expect_token(lexer, TokenType::Identifier)?;
    let name = lexer.curr_token.lexeme.clone();
    lexer.consume_token();
    expect_token(lexer, TokenType::OpenBrace)?;
    lexer.consume_token();
    let def = parse_ComponentBody(lexer, name)?;
    expect_token(lexer, TokenType::CloseBrace)?;
    lexer.consume_token();
    expect_token(lexer, TokenType::Terminator)?;
    lexer.consume_token();
    Ok(def)
}

/*
ComponentInstance ->
    Keyword("create") Identifier(*) Terminator(";")
    | Keyword("create") Identifier(*) Operator(":") Identifier(*) Terminator(";")
    | Keyword("create") Identifier(*) Operator(":") Identifier(*) Operator("=") String(*) Terminator(";")
*/

#[allow(non_snake_case)]
fn parse_ComponentInstance(lexer: &mut Lexer) -> Result<ComponentInstance, Error> {
    expect(lexer, TokenType::Keyword, "create")?;
    lexer.consume_token();
    expect_token(lexer, TokenType::Identifier)?;
    let component = lexer.curr_token.lexeme;
    let mut inst = ComponentInstance {
        component: component,
        identifier: None,
        value: None,
    };
    lexer.consume_token();
    match lexer.curr_token.t {
        TokenType::Terminator => {
            lexer.consume_token();
            Ok(inst.clone())
        }
        TokenType::Operator => {
            expect(lexer, TokenType::Operator, ":")?;
            lexer.consume_token();
            expect_token(lexer, TokenType::Identifier)?;
            inst.name = lexer.curr_token.lexeme;
        }
        _ => Err(Error::new(ErrorKind::Other, "invalid component instance")),
    }
}

/*
Assignment ->
    Identifier(*) Operator(".") Identifier(*) Operator("=") String(*) Terminator(";")
*/

// Wtf am i doing bro
/*
#[allow(non_snake_case)]
pub fn parse_ComponentDefinition(lexer: &mut Lexer) -> Result<ComponentDefinition, Error> {
    match lexer.curr_token.t {
        TokenType::Keyword => match lexer.curr_token.lexeme.as_str() {
            "create" => {
                lexer.consume_token();
                // keyword(component)
                match lexer.curr_token.t {
                    TokenType::Keyword => match lexer.curr_token.lexeme.as_str() {
                        "component" => {
                            lexer.consume_token();
                            // Operator
                            match lexer.curr_token.t {
                                TokenType::Operator => match lexer.curr_token.lexeme.as_str() {
                                    ":" => {
                                        lexer.consume_token();
                                        // component name (Identifier)
                                        match lexer.curr_token.t {
                                            TokenType::Identifier => {
                                                let n = lexer.curr_token.lexeme.clone();
                                                lexer.consume_token();
                                                // block
                                                match lexer.curr_token.t {
                                                    TokenType::OpenBrace => {
                                                        lexer.consume_token();
                                                        match parse_ComponentBody(lexer, n) {
                                                            Ok(s) => match lexer.curr_token.t {
                                                                TokenType::CloseBrace => {
                                                                    lexer.consume_token();
                                                                    match lexer.curr_token.t {
                                                                        TokenType::Terminator => {
                                                                            lexer.consume_token();
                                                                            // let mut def = ComponentDefinition {
                                                                            //     name: n,
                                                                            //     vars: Vec::new(),
                                                                            //     default_var: None,
                                                                            //     html: None,
                                                                            //     css: None,
                                                                            //     js: None,
                                                                            // };
                                                                            Ok(s)
                                                                        }
                                                                        _ => Err(Error::new(
                                                                            ErrorKind::Other,
                                                                            "unterminated",
                                                                        )),
                                                                    }
                                                                }
                                                                _ => Err(Error::new(
                                                                    ErrorKind::Other,
                                                                    "not a block",
                                                                )),
                                                            },
                                                            Err(E) => Err(E),
                                                        }
                                                    }
                                                    _ => Err(Error::new(
                                                        ErrorKind::Other,
                                                        "not a block",
                                                    )),
                                                }
                                            }
                                            _ => Err(Error::new(
                                                ErrorKind::Other,
                                                "not a identifier",
                                            )),
                                        }
                                    }
                                    _ => Err(Error::new(ErrorKind::Other, "expected ':'")),
                                },
                                _ => Err(Error::new(ErrorKind::Other, "not a operator")),
                            }
                        }
                        _ => Err(Error::new(ErrorKind::Other, "expected 'component'")),
                    },
                    _ => Err(Error::new(ErrorKind::Other, "not a keyword")),
                }
            }
            _ => Err(Error::new(ErrorKind::Other, "expected 'create'")),
        },
        _ => Err(Error::new(ErrorKind::Other, "not a keyword")),
    }
}
*/

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
