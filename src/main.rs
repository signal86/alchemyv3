use std::env;
use std::fs::File;
use std::io;
mod lexer;
mod parser;
use parser::parser::parse_file;
// use std::io::{BufRead, BufReader};
// mod lexer;
// use lexer::lexer::lex;
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
    parse_file(&file);

    // let r = BufReader::new(file);
    // for (_, line) in r.lines().enumerate() {
    //     let l: String = line?;
    //     let mut tokens: Vec<Vec<Token>> = Vec::new();
    //     tokens.push(lex(&l));
    //     println!("{}", l);
    //     println!("{:#?}", lex(&l));
    // }

    Ok(())
}
