use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn comment() {
    let results = scan_with_whitespace("// line comment yo 123\n//another line comment", true);
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::Comment,
                lexeme: "// line comment yo 123".to_string(),
                line: 1,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::Comment,
                lexeme: "//another line comment".to_string(),
                line: 2,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 2,
                column: 23,
            },
        ]
    );
}

#[test]
fn block_comment() {
    let results = scan_with_whitespace(
        r#"/*print this
is a test string*/"#,
        true,
    );
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::BlockComment,
                lexeme: r#"/*print this
is a test string*/"#
                    .to_string(),
                line: 1,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 2,
                column: 19,
            },
        ]
    )
}
