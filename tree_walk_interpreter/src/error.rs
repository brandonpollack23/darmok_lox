use std::num::ParseFloatError;

use thiserror::Error;

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Error, Clone, Debug)]
pub enum LoxError {
    #[error("Error at {0}:{1} Unexpected character '{2}'")]
    UnexpectedCharacter(usize, usize, char),
    #[error("Error at {0}:{1} String was unterminated")]
    UnterminatedString(usize, usize),
    #[error("Error at {0}:{1} String contained an unknown escape sequence: \"{2}\"")]
    UnknownStringEscapeSequence(usize, usize, String),

    #[error("Error at {0}:{1} Unable to Parse float: {2:?}")]
    UnableToParseNumber(usize, usize, ParseFloatError),
}
