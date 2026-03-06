use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod lexizer;
use lexizer::lexizer::lex;
use lexizer::token::Token;

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

    for (i, line) in r.lines().enumerate() {
        let l: String = line?;
        let mut tokens: Vec<Vec<Token>> = Vec::new();
        // for tokens_line in lex(&l, (i + 1) as u128) {
        tokens.push(lex(&l, (i + 1) as u128));
        // }
        // println!("{}", l);
        // println!("{:#?}", lex(&l, (i + 1) as u128));
    }

    Ok(())
}
