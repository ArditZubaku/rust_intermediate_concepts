use std::{
    fs::File,
    io::{self, IsTerminal},
};

// use std::{
//     error::Error,
//     fmt::{Display, Formatter},
// };
use thiserror::Error;

#[allow(dead_code)]
fn main() {
    // Best practices for enums
    // Errors "should" be enums
    // Group Errors
    // Expose only your errors
    // Add the non_exhaustive attribute
    // Implement Debug, Display, Error (in that order) <- NOTE: Instead of doing this use thiserror

    #[derive(Debug, Error)]
    #[non_exhaustive]
    pub enum PuzzleError {
        #[error("Piece {0} doesn't fit")]
        WontFit(u16),
        #[error("Missing a piece")]
        MissingPiece,
    }

    // impl Display for PuzzleError {
    //     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    //         use PuzzleError::*;
    //
    //         match self {
    //             WontFit(n) => write!(f, "Piece {} doesn't fit", n),
    //             MissingPiece => write!(f, "Missing a piece"),
    //         }
    //     }
    // }
    //
    // // That's all it's needed
    // impl Error for PuzzleError {}

    let my_result: Result<i32, String> = Err("Something went wrong".to_string());

    if let Err(e) = my_result {
        println!("Warning: {}", e)
    }

    fn get_saved_score() -> Result<i8, String> {
        Err("Something went wrong".to_string())
    }

    #[allow(clippy::manual_unwrap_or)]
    // Both cases should return the same type
    let score = match get_saved_score() {
        Ok(n) => n,
        Err(_) => -1,
    };
    println!("Score {}", score);

    fn poem() -> Result<bool, io::Error> {
        // The `?` operator basically means assign the success value to file or
        // return the error for the whole function
        let file = File::open("/tmp/bwlint.log")?;

        println!("{}", file.is_terminal());

        Ok(file.is_terminal())
    }
}
