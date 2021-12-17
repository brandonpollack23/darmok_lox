use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn whitespace_ignored() {
    let results = scan_with_whitespace("print \"this is a test string\"", true);
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::Print,
                lexeme: "print".to_string(),
                line: 1,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::String("this is a test string".to_string()),
                lexeme: "\"this is a test string\"".to_string(),
                line: 1,
                column: 7,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 30,
            },
        ]
    )
}
