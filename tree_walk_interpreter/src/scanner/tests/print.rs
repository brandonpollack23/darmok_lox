use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn print() {
    let results = scan_with_whitespace("print \"this is a test string\"", false);
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
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 6,
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

#[test]
fn print_supports_newline() {
    let results = scan_with_whitespace(
        r#"print "this is a
test string" "#,
        false,
    );
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
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 6,
            },
            LoxToken {
                token_type: TokenType::String(
                    r#"this is a
test string"#
                        .to_string()
                ),
                lexeme: r#""this is a
test string""#
                    .to_string(),
                line: 1,
                column: 7,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 2,
                column: 13,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 2,
                column: 14,
            },
        ]
    )
}

#[test]
fn print_escapes() {
    let results = scan_with_whitespace("print \"\\\\this \\tis a\\n test string\\n\"", false);
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
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 6,
            },
            LoxToken {
                token_type: TokenType::String("\\\\this \\tis a\\n test string\\n".to_string()),
                lexeme: "\"\\\\this \\tis a\\n test string\\n\"".to_string(),
                line: 1,
                column: 7,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 38,
            },
        ]
    )
}
