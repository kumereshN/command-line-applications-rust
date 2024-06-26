
use anyhow::{Context, Result, Error};
use crate::find_matches;

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}

