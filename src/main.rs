use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read};
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path).with_context(|| format!("Could not find file {:?}", args.path))?;

    let mut reader = BufReader::new(f);
    let mut lines = String::new();
    reader
        .read_to_string(&mut lines)
        .with_context(|| format!("could not read file {:?}", args.path))?;

    for line in lines.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }

    Ok(())
}
