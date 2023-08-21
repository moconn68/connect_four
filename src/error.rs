use crate::board::InsertError;

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Top-level errors that may occur during gameplay.
/// All other error types should convert into this for handling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameError {
    AddPiece(InsertError),
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GameError::AddPiece(insert_error) => {
                write!(f, "Error placing new piece: {}", insert_error)
            }
        }
    }
}

// Conversions for game logic error types into GameError

impl From<InsertError> for GameError {
    fn from(value: InsertError) -> Self {
        Self::AddPiece(value)
    }
}
