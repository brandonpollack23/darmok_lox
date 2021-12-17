use pretty_assertions::assert_eq;

use crate::scanner::scan_with_whitespace;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::LoxResult;

#[test]
fn builtins() {
    let results = scan_with_whitespace(
        "and class else false for fun if nil or print return super this true var while",
        false,
    );
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::And,
                lexeme: "and".to_string(),
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
                token_type: TokenType::Class,
                lexeme: "class".to_string(),
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
                token_type: TokenType::Else,
                lexeme: "else".to_string(),
                line: 1,
                column: 11,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 15,
            },
            LoxToken {
                token_type: TokenType::False,
                lexeme: "false".to_string(),
                line: 1,
                column: 16,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 21,
            },
            LoxToken {
                token_type: TokenType::For,
                lexeme: "for".to_string(),
                line: 1,
                column: 22,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 25,
            },
            LoxToken {
                token_type: TokenType::Fun,
                lexeme: "fun".to_string(),
                line: 1,
                column: 26,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 29,
            },
            LoxToken {
                token_type: TokenType::If,
                lexeme: "if".to_string(),
                line: 1,
                column: 30,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 32,
            },
            LoxToken {
                token_type: TokenType::Nil,
                lexeme: "nil".to_string(),
                line: 1,
                column: 33,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 36,
            },
            LoxToken {
                token_type: TokenType::Or,
                lexeme: "or".to_string(),
                line: 1,
                column: 37,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 39,
            },
            LoxToken {
                token_type: TokenType::Print,
                lexeme: "print".to_string(),
                line: 1,
                column: 40,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 45,
            },
            LoxToken {
                token_type: TokenType::Return,
                lexeme: "return".to_string(),
                line: 1,
                column: 46,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 52,
            },
            LoxToken {
                token_type: TokenType::Super,
                lexeme: "super".to_string(),
                line: 1,
                column: 53,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 58,
            },
            LoxToken {
                token_type: TokenType::This,
                lexeme: "this".to_string(),
                line: 1,
                column: 59,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 63,
            },
            LoxToken {
                token_type: TokenType::True,
                lexeme: "true".to_string(),
                line: 1,
                column: 64,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 68,
            },
            LoxToken {
                token_type: TokenType::Var,
                lexeme: "var".to_string(),
                line: 1,
                column: 69,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 72,
            },
            LoxToken {
                token_type: TokenType::While,
                lexeme: "while".to_string(),
                line: 1,
                column: 73,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 78,
            },
        ]
    );
}

#[test]
fn non_builtins() {
    let results = scan_with_whitespace("Resistance is futile", false);
    let tokens: LoxResult<Vec<LoxToken>> = results.into_iter().collect();
    assert!(tokens.is_ok());
    assert_eq!(
        tokens.unwrap(),
        vec![
            LoxToken {
                token_type: TokenType::Identifier,
                lexeme: "Resistance".to_string(),
                line: 1,
                column: 1,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 11,
            },
            LoxToken {
                token_type: TokenType::Identifier,
                lexeme: "is".to_string(),
                line: 1,
                column: 12,
            },
            LoxToken {
                token_type: TokenType::Space,
                lexeme: " ".to_string(),
                line: 1,
                column: 14,
            },
            LoxToken {
                token_type: TokenType::Identifier,
                lexeme: "futile".to_string(),
                line: 1,
                column: 15,
            },
            LoxToken {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
                column: 21,
            },
        ]
    );
}
