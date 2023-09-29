use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("failed to open file `{}`", &args.path.display()))?;
    let mut reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line.trim()).unwrap();
        }

        line.clear();
    }

    handle.flush().unwrap();

    Ok(())
}
