use pretty_assertions::assert_eq;

use crate::error::LoxError;
use crate::error::ScannerError::{UnexpectedCharacter, UnterminatedString};
use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn unexpected_char() {
    let results = scan_with_whitespace("123 ~ 123.0 0.123", false);
    assert_eq!(
        results,
        vec![
            Ok(LoxToken {
                token_type: TokenType::Number(123.0f64),
                lexeme: "123".to_string(),
                line: 1,
                column: 1,
            }),
            Ok(LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 4,
            }),
            Err(LoxError::ScannerError(UnexpectedCharacter(1, 5, '~'))),
            Ok(LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 6,
            }),
            Ok(LoxToken {
                token_type: TokenType::Number(123.0f64),
                lexeme: "123.0".to_string(),
                line: 1,
                column: 7,
            }),
            Ok(LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 12,
            }),
            Ok(LoxToken {
                token_type: TokenType::Number(0.123f64),
                lexeme: "0.123".to_string(),
                line: 1,
                column: 13,
            }),
            Ok(LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 18,
            }),
        ]
    );
}

#[test]
fn unterminated_string() {
    let results = scan_with_whitespace("\"locutus", false);
    assert_eq!(
        results,
        vec![
            Err(LoxError::ScannerError(UnterminatedString(1, 1))),
            Ok(LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 10,
            }),
        ]
    );
}
