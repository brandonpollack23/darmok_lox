use crate::consume_single_char_token;
use crate::error::{LoxError, LoxResult};
use crate::scanner::escapable_string::UnEscapableString;

mod escapable_string;
mod macros;

// BONUS string interpolation by making double quote a token on its own.

/// Scan the source expression and return it as a list of [LoxTokens](LoxToken), dropping whitespace if requested.
pub fn scan_with_whitespace(source: &str, remove_whitespace: bool) -> Vec<LoxResult<LoxToken>> {
    let tokens = Tokenizer::new().tokenize(source);
    if remove_whitespace {
        tokens
            .into_iter()
            .filter(|tr| !tr.as_ref().map(|t| t.is_whitespace()).unwrap_or(true))
            .collect()
    } else {
        tokens
    }
}

/// Scan the source expression and return it as a list of [LoxTokens](LoxToken), removing all whitespace.
pub fn scan(source: &str) -> Vec<LoxResult<LoxToken>> {
    scan_with_whitespace(source, true)
}

/// Tokenized representation of Lox source code
#[derive(Clone, Debug)]
pub struct LoxToken {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
}

impl LoxToken {
    fn is_whitespace(&self) -> bool {
        match self.token_type {
            TokenType::Space | TokenType::Linefeed | TokenType::CarriageReturn | TokenType::Tab => {
                true
            }
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    // Pruned tokens such as whitespace/comments that are useful for linting.
    Space,
    Linefeed,
    CarriageReturn,
    Comment,
    Tab,

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
            // Ignored single characters that are added for linting purposes.
            ' ' => consume_single_char_token!(state, first, Space),
            '\r' => consume_single_char_token!(state, first, CarriageReturn),
            '\n' => consume_single_char_token!(state, first, Linefeed),
            '\t' => consume_single_char_token!(state, first, Tab),

            // Unambiguous single characters.
            '(' => consume_single_char_token!(state, first, LeftParen),
            ')' => consume_single_char_token!(state, first, RightParen),
            '{' => consume_single_char_token!(state, first, LeftBrace),
            '}' => consume_single_char_token!(state, first, RightBrace),
            ',' => consume_single_char_token!(state, first, Comma),
            '.' => consume_single_char_token!(state, first, Dot),
            '-' => consume_single_char_token!(state, first, Minus),
            '+' => consume_single_char_token!(state, first, Plus),
            ';' => consume_single_char_token!(state, first, Semicolon),
            '*' => consume_single_char_token!(state, first, Star),

            // Single characters that may have more chars
            '!' => {
                let (token, next_state) = Self::consume_ambiguous_single_char_token(state, first);
                (Ok(token), next_state)
            }
            '/' => {
                // Special handling because of comments.
                let (token, next_state) = Self::consume_lexeme_beginning_with_forward_slash(state);
                (Ok(token), next_state)
            }

            // Multi char tokens
            '"' => Self::consume_string(state),
            _ => (
                Err(LoxError::UnexpectedCharacter(
                    state.line,
                    state.column,
                    first,
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
        let is_new_line = token_type == TokenType::Linefeed;
        (
            LoxToken {
                token_type,
                lexeme: first.to_string(),
                line: state.line,
                column: state.column,
            },
            if is_new_line {
                state.consume_newline()
            } else {
                state.consume_single_char()
            },
        )
    }

    /// Lexemes beginning with ! = < > (and maybe more in the future) are ambiguous and may be
    /// longer than just one char, so we check.
    fn consume_ambiguous_single_char_token<'a, 'b>(
        state: &'a TokenizerState<'b>,
        first: char,
    ) -> (LoxToken, TokenizerState<'b>) {
        if !Self::second_char_matches(state, '=') {
            let token_type = Self::get_disambiguated_single_char_lexeme(first);
            return (
                LoxToken {
                    token_type,
                    lexeme: first.to_string(),
                    line: state.line,
                    column: state.column,
                },
                state.consume_single_char(),
            );
        }

        let token_type = match first {
            '!' => TokenType::BangEqual,
            '=' => TokenType::EqualEqual,
            '<' => TokenType::LessEqual,
            '>' => TokenType::GreaterEqual,
            _ => panic!("This character: {} is not ambiguous!", first),
        };
        (
            LoxToken {
                token_type,
                lexeme: state.remaining[0..2].to_string(),
                line: state.line,
                column: state.column,
            },
            state.consume_n_chars(2),
        )
    }

    fn consume_lexeme_beginning_with_forward_slash<'a, 'b>(
        state: &'a TokenizerState<'b>,
    ) -> (LoxToken, TokenizerState<'b>) {
        if Self::second_char_matches(state, '/') {
            // This is a line comment
            let comment_line: String = state.remaining.chars().take_while(|&c| c != '\n').collect();
            let comment_length = comment_line.len();
            return (
                LoxToken {
                    token_type: TokenType::Comment,
                    lexeme: comment_line,
                    line: state.line,
                    column: state.column,
                },
                state.consume_n_chars(comment_length),
            );
        }

        (
            LoxToken {
                token_type: TokenType::Slash,
                lexeme: '/'.to_string(),
                line: state.line,
                column: state.column,
            },
            state.consume_single_char(),
        )
    }

    // TODO NOW escape sequences \
    fn consume_string<'a, 'b>(
        state: &'a TokenizerState<'b>,
    ) -> (LoxResult<LoxToken>, TokenizerState<'b>) {
        let string_without_initial_quote: String = state
            .remaining
            .chars()
            .skip(1)
            .take_while(|&c| c != '"')
            .collect();

        let num_newlines = string_without_initial_quote
            .chars()
            .filter(|&x| x == '\n')
            .count();
        let string_terminated = string_without_initial_quote.chars().last().unwrap() == '"';

        if !string_terminated {
            return (
                Err(LoxError::UnterminatedString(state.line, state.column)),
                state.consume_n_chars_with_newlines(
                    string_without_initial_quote.len() + 1,
                    num_newlines,
                ),
            );
        }
        let string = format!("{}{}", '"', string_without_initial_quote);

        string[1..string.len()]
            .unescape_string(state.line, state.column)
            .map(|s| {
                (
                    Ok(LoxToken {
                        token_type: TokenType::String,
                        lexeme: s,
                        line: state.line,
                        column: state.column,
                    }),
                    state.consume_n_chars_with_newlines(string.len(), num_newlines),
                )
            })
            .unwrap_or_else(|e| {
                (
                    Err(e),
                    state.consume_n_chars_with_newlines(string.len(), num_newlines),
                )
            })
    }

    /// Converts chars that may have another char to their lexeme to their [TokenType] when they
    /// don't.
    fn get_disambiguated_single_char_lexeme(ch: char) -> TokenType {
        match ch {
            '!' => TokenType::Bang,
            '=' => TokenType::Equal,
            '<' => TokenType::Less,
            '>' => TokenType::Greater,
            _ => panic!("This character: {} is not ambiguous!", ch),
        }
    }

    fn second_char_matches(state: &TokenizerState, ch: char) -> bool {
        !state.remaining.len() > 1 && state.remaining.chars().nth(1).unwrap() == ch
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

    fn consume_newline(self) -> TokenizerState<'a> {
        TokenizerState {
            column: 1,
            line: self.line + 1,
            remaining: &self.remaining[1..],
        }
    }

    fn consume_single_char(self) -> TokenizerState<'a> {
        self.consume_n_chars(1)
    }

    fn consume_n_chars(self, n: usize) -> TokenizerState<'a> {
        self.consume_n_chars_with_newlines(n, 0)
    }

    fn consume_n_chars_with_newlines(self, n: usize, newlines: usize) -> TokenizerState<'a> {
        TokenizerState {
            column: self.column + n,
            remaining: &self.remaining[n..],
            line: self.line + newlines,
            ..self
        }
    }
}
