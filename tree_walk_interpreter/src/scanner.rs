use regex::Regex;

use crate::error::{LoxError, LoxResult};

/// Scan the source expression and return it as a list of [LoxTokens](LoxToken)
pub fn scan(source: &str) -> LoxResult<Vec<LoxToken>> {
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

    fn tokenize(&self, input: &str) -> LoxResult<Vec<LoxToken>> {
        let mut tokens = Vec::new();
        let mut tokenizer_state = TokenizerState::new(input);

        while tokenizer_state.remaining.len() > 0 {
            let (token, tokenizer_state) = self.tokenize_next(&tokenizer_state)?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn tokenize_next<'a, 'b>(&self, state: &'a TokenizerState<'b>) -> LoxResult<(LoxToken, TokenizerState<'b>)> {
        let first = state.remaining.chars().nth(0).unwrap();
        match first {
            '(' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::LeftParen)),
            ')' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::RightParen)),
            '{' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::LeftBrace)),
            '}' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::RightBrace)),
            ',' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Comma)),
            '.' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Dot)),
            '-' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Minus)),
            '+' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Plus)),
            ';' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Semicolon)),
            '*' =>
                Ok(Self::consume_single_char_token(state, first, TokenType::Star)),
            _ => Err(LoxError::UnexpectedCharacter(first, state.line, state.column)),
        }
    }

    fn consume_single_char_token<'a, 'b>(state: &'a TokenizerState<'b>, first: char, token_type: TokenType) -> (LoxToken, TokenizerState<'b>) {
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
