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
}
