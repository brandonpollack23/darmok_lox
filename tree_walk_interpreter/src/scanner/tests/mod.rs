use pretty_assertions::assert_eq;

use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

mod comments;
mod digits;
mod identifiers;
mod print;
mod token_error;
mod whitespace;
