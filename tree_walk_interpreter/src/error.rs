use std::num::ParseFloatError;

use thiserror::Error;

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum LoxError {
    #[error("{0:?}")]
    ScannerError(#[from] ScannerError),

    #[error("{0:?}")]
    LinterError(#[from] LinterError),
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum ScannerError {
    #[error("Error at {0}:{1} Unexpected character '{2}'")]
    UnexpectedCharacter(usize, usize, char),
    #[error("Error at {0}:{1} String was unterminated")]
    UnterminatedString(usize, usize),
    #[error("Error at {0}:{1} String contained an unknown escape sequence: \"{2}\"")]
    UnknownStringEscapeSequence(usize, usize, String),

    #[error("Error at {0}:{1} Unable to Parse float: {2:?}")]
    UnableToParseNumber(usize, usize, ParseFloatError),
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum LinterError {
    #[error("Linter Error {0}:{1} More than one space detected, standard style is to only have one space")]
    DoubleSpaceDetected(usize, usize),
}
