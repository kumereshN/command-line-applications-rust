mod test;

use std::io::Write;
use anyhow::{Context};

pub fn find_matches(content: &str, pattern: &str, writer: &mut impl Write) -> anyhow::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).context("Unable to write to buffer")?;
        }
    }
    Ok(())
}