use regex::Regex;

use crate::error::{LoxError, LoxResult};

/// Scan the source expression and return it as a list of [LoxTokens](LoxToken)
pub fn scan(source: &str) -> Vec<LoxResult<LoxToken>> {
    Tokenizer::new().tokenize(source)
}

/// Tokenized representation of Lox source code
#[derive(Clone, Debug)]
pub struct LoxToken {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

/// A struct that caches the regex expressions for tokenizing
struct Tokenizer {}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {}
    }

    fn tokenize(&self, input: &str) -> Vec<LoxResult<LoxToken>> {
        let mut tokens: Vec<LoxResult<LoxToken>> = Vec::new();
        let mut tokenizer_state = TokenizerState::new(input);

        while tokenizer_state.remaining.len() > 0 {
            let (token, next_state) = self.tokenize_next(&tokenizer_state);
            tokens.push(token);
            tokenizer_state = next_state;
        }

        tokens
    }

    fn tokenize_next<'a, 'b>(
        &self,
        state: &'a TokenizerState<'b>,
    ) -> (LoxResult<LoxToken>, TokenizerState<'b>) {
        let first = state.remaining.chars().nth(0).unwrap();
        match first {
            '(' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::LeftParen);
                (Ok(token), next_state)
            }
            ')' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::RightParen);
                (Ok(token), next_state)
            }
            '{' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::LeftBrace);
                (Ok(token), next_state)
            }
            '}' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::RightBrace);
                (Ok(token), next_state)
            }
            ',' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Comma);
                (Ok(token), next_state)
            }
            '.' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Dot);
                (Ok(token), next_state)
            }
            '-' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Minus);
                (Ok(token), next_state)
            }
            '+' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Plus);
                (Ok(token), next_state)
            }
            ';' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Semicolon);
                (Ok(token), next_state)
            }
            '*' => {
                let (token, next_state) =
                    Self::consume_single_char_token(state, first, TokenType::Star);
                (Ok(token), next_state)
            }

            _ => (
                Err(LoxError::UnexpectedCharacter(
                    first,
                    state.line,
                    state.column,
                )),
                state.consume_single_char(),
            ),
        }
    }

    fn consume_single_char_token<'a, 'b>(
        state: &'a TokenizerState<'b>,
        first: char,
        token_type: TokenType,
    ) -> (LoxToken, TokenizerState<'b>) {
        (
            LoxToken {
                token_type,
                lexeme: first.to_string(),
                line: state.line,
                column: state.column,
            },
            state.consume_single_char(),
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct TokenizerState<'a> {
    line: usize,
    column: usize,
    remaining: &'a str,
}

impl<'a> TokenizerState<'a> {
    fn new(input: &str) -> TokenizerState {
        TokenizerState {
            line: 0,
            column: 0,
            remaining: input,
        }
    }

    fn consume_single_char(self) -> TokenizerState<'a> {
        TokenizerState {
            column: self.column + 1,
            remaining: &self.remaining[1..],
            ..self
        }
    }
}
