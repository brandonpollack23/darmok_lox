use pretty_assertions::assert_eq;

use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn digits() {
    let results = scan_with_whitespace("123 123.0 0.123", false);
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::Number(123.0f64),
                lexeme: "123".to_string(),
                line: 1,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 4,
            },
            LoxToken {
                token_type: TokenType::Number(123.0f64),
                lexeme: "123.0".to_string(),
                line: 1,
                column: 5,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 10,
            },
            LoxToken {
                token_type: TokenType::Number(0.123f64),
                lexeme: "0.123".to_string(),
                line: 1,
                column: 11,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 16,
            },
        ]
    );
}
