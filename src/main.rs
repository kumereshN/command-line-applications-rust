
use std::fs::File;
use std::io::{stdout,BufReader, Read};
use anyhow::{Context, Result};
use log::{info, warn};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String
}



fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let args = Cli::parse();
    let f = File::open(&args.path).with_context(|| format!("Could not find file {:?}", args.path))?;

    let mut reader = BufReader::new(f);
    let mut lines = String::new();
    reader
        .read_to_string(&mut lines)
        .with_context(|| format!("could not read file {:?}", args.path))?;

    let pattern = "main";
    grrs::find_matches(lines.as_str(), pattern, &mut stdout())
        .context("Some issue with finding matches")?;

/*    let stdout = stdout();
    let mut handle = BufWriter::new(stdout);
    writeln!(handle, "foo: {}", 42)?;

    let stdout = stdout;
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42)?;*/

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+]finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}
