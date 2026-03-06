use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("alchemy <file>");
        std::process::exit(1);
    }

    for (i, arg) in args.iter().enumerate() {
        println!("{}: {}", i, arg);
    }

    let fname = &args[1];
    let file = File::open(fname)?;
    let r = BufReader::new(file);

    for line in r.lines() {
        println!("{}", line?);
    }

    Ok(())
}
