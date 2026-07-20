use anyhow::{Context, Ok, Result};
use puzzles::Puzzle;
use std::fs::File;

fn main() -> Result<()> {
    // Nobody handles errors, we just pass them up the call stack
    let puzzle = get_puzzle("puzzle.dat").context("couldn't get the first puzzle")?;
    println!("Playing puzzle: {}", puzzle.name);

    Ok(())
}

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    // let file = File::open(filename).context("couldn't open the puzzle file")?;
    let file = File::open(filename)
        .with_context(|| format!("couldn't open the puzzle file {}", filename))?;

    let puzzle = Puzzle::from_file(file).context("couldn't convert data into a puzzle")?;

    Ok(puzzle)
}
