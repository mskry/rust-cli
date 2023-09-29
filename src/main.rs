use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.path).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
        line.clear();
    }
}
