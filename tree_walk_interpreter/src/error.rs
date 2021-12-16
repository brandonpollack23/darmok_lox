use thiserror::Error;

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Error, Clone, Debug)]
pub enum LoxError {
    #[error("Error at {1}:{2} Unexpected character '{0}'")]
    UnexpectedCharacter(char, usize, usize),
}