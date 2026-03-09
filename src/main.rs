use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod parser;
use parser::parser::parse_statement;
mod lexer;
use lexer::lexer::lex;
// use lexer::token::Token;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("alchemy <file>");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "no parameters supplied",
        ))?;
    }

    // for (i, arg) in args.iter().enumerate() {
    // println!("{}: {}", i, arg);
    // }

    let fname = &args[1];
    let file = File::open(fname)?;
    let r = BufReader::new(file);

    let mut context: Vec<String> = Vec::new();

    for (_, line) in r.lines().enumerate() {
        let l: String = line?;
        // let mut tokens: Vec<Vec<Token>> = Vec::new();
        // for tokens_line in lex(&l, (i + 1) as u128) {
        // tokens.push(lex(&l));
        let new_ctx = parse_statement(&context, lex(&l));
        match new_ctx {
            Some(s) => context.push(s),
            None => {}
        }
        // }
        // println!("{}", l);
        // println!("{:#?}", lex(&l, (i + 1) as u128));
    }

    Ok(())
}
