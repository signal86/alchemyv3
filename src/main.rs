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

    for (i, arg) in args.iter().enumerate() {
        println!("{}: {}", i, arg);
    }

    let fname = &args[1];
    let file = File::open(fname)?;
    let r = BufReader::new(file);

    for line in r.lines() {
        let l: String = line?;
        println!("{}", l);
        println!("{:#?}", lex(&l));
    }

    Ok(())
}
