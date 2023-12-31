use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BrainFuckError {
    SyntaxError,
    PointerOutOfBounds,
    IncorrectValue,
    NoInputFound,
}

impl fmt::Display for BrainFuckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrainFuckError::SyntaxError => write!(f, "SYNTAX ERROR"),
            BrainFuckError::PointerOutOfBounds => write!(f, "POINTER OUT OF BOUNDS"),
            BrainFuckError::IncorrectValue => write!(f, "INCORRECT VALUE"),
            BrainFuckError::NoInputFound => write!(f, "NO INPUT FOUND"),
        }
    }
}
