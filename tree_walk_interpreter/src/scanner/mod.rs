use std::str::FromStr;

use phf::phf_map;

use crate::consume_single_char_token;
use crate::error::{LoxError, LoxResult};
use crate::scanner::escapable_string::UnEscapableString;
use crate::scanner::tokens::{LoxToken, TokenType};
use crate::utils::{is_alpha, is_alpha_numeric, is_digit};

pub mod tokens;

mod escapable_string;
mod macros;

// BONUS string interpolation by making double quote a token on its own.

/// Scan the source expression and return it as a list of [LoxTokens](LoxToken), dropping whitespace if requested.
pub fn scan_with_whitespace(source: &str, remove_whitespace: bool) -> Vec<LoxResult<LoxToken>> {
    let tokens = tokenize(source);
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
/// A struct that caches the regex expressions for tokenizing

fn tokenize(input: &str) -> Vec<LoxResult<LoxToken>> {
    let mut tokens: Vec<LoxResult<LoxToken>> = Vec::new();
    let mut tokenizer_state = TokenizerState::new(input);

    while tokenizer_state.remaining.len() > 0 {
        let (token, next_state) = tokenize_next(&tokenizer_state);
        tokens.push(token);
        tokenizer_state = next_state;
    }

    tokens.push(Ok(LoxToken {
        token_type: TokenType::EOF,
        lexeme: "".to_string(),
        line: tokenizer_state.line,
        column: tokenizer_state.column,
    }));
    tokens
}

fn tokenize_next<'a, 'b>(
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
            let (token, next_state) = consume_ambiguous_single_char_token(state, first);
            (Ok(token), next_state)
        }
        '/' => {
            // Special handling because of comments.
            let (token, next_state) = consume_lexeme_beginning_with_forward_slash(state);
            (Ok(token), next_state)
        }

        // Multi char tokens
        '"' => consume_string(state),
        d if is_digit(d) => consume_digit(state),

        // Keywords/identifiers, require maximal munching.
        c if is_alpha(c) => consume_identifier(state),

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
    if !second_char_matches(state, '=') {
        let token_type = get_disambiguated_single_char_lexeme(first);
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
    if second_char_matches(state, '/') {
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

fn consume_string<'a, 'b>(
    state: &'a TokenizerState<'b>,
) -> (LoxResult<LoxToken>, TokenizerState<'b>) {
    let string_without_quotes: String = state
        .remaining
        .chars()
        .skip(1)
        .take_while(|&c| c != '"')
        .collect();

    let num_newlines = string_without_quotes.chars().filter(|&x| x == '\n').count();
    let string_terminated = state.remaining[1..=string_without_quotes.len() + 1]
        .chars()
        .last()
        .map(|c| c == '"')
        .unwrap_or(false);

    if !string_terminated {
        return (
            Err(LoxError::UnterminatedString(state.line, state.column)),
            state.consume_n_chars_with_newlines(string_without_quotes.len() + 1, num_newlines),
        );
    }
    let string = format!("\"{}\"", string_without_quotes);
    let chars_to_consume = string.len();

    string_without_quotes
        .unescape_string(state.line, state.column)
        .map(|s| {
            (
                Ok(LoxToken {
                    token_type: TokenType::String(s),
                    lexeme: string,
                    line: state.line,
                    column: state.column,
                }),
                state.consume_n_chars_with_newlines(chars_to_consume, num_newlines),
            )
        })
        .unwrap_or_else(|e| {
            (
                Err(e),
                state.consume_n_chars_with_newlines(chars_to_consume, num_newlines),
            )
        })
}

// BONUS: support numbers without 0. such as .5
fn consume_digit<'a, 'b>(
    state: &'a TokenizerState<'b>,
) -> (LoxResult<LoxToken>, TokenizerState<'b>) {
    let mut result: String = state
        .remaining
        .chars()
        .take_while(|&d| is_digit(d))
        .collect();
    if nth_char_matches(state, result.len(), '.')
        && nth_char_matches_fn(state, result.len() + 1, is_digit)
    {
        result.push('.');
        result.extend(
            state.remaining[result.len() + 1..]
                .chars()
                .take_while(|&d| is_digit(d)),
        );
    }

    let chars_to_consume = result.len();
    let parsed_result = f64::from_str(&result);
    (
        parsed_result
            .map(|n| LoxToken {
                token_type: TokenType::Number(n),
                lexeme: result,
                line: state.line,
                column: state.column,
            })
            .map_err(|e| LoxError::UnableToParseNumber(state.line, state.column, e)),
        state.consume_n_chars(chars_to_consume),
    )
}

fn consume_identifier<'a, 'b>(
    state: &'a TokenizerState<'b>,
) -> (LoxResult<LoxToken>, TokenizerState<'b>) {
    let identifier: String = state
        .remaining
        .chars()
        .take_while(|&c| is_alpha_numeric(c))
        .collect();

    let token_type = KEYWORDS
        .get(&identifier)
        .cloned()
        .unwrap_or(TokenType::Identifier);
    let chars_to_consume = identifier.len();

    (
        Ok(LoxToken {
            token_type,
            lexeme: identifier,
            line: state.line,
            column: state.column,
        }),
        state.consume_n_chars(chars_to_consume),
    )
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

fn nth_char_matches_fn<F>(state: &TokenizerState, n: usize, f: F) -> bool
where
    F: FnOnce(char) -> bool,
{
    !state.remaining.len() > n && f(state.remaining.chars().nth(n).unwrap())
}

fn nth_char_matches(state: &TokenizerState, n: usize, ch: char) -> bool {
    nth_char_matches_fn(state, n, |c| c == ch)
}

fn second_char_matches(state: &TokenizerState, ch: char) -> bool {
    nth_char_matches(state, 1, ch)
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
            line: 1,
            column: 1,
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

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" =>    TokenType::And,
    "class" =>  TokenType::Class,
    "else" =>   TokenType::Else,
    "false" =>  TokenType::False,
    "for" =>    TokenType::For,
    "fun" =>    TokenType::Fun,
    "if" =>     TokenType::If,
    "nil" =>    TokenType::Nil,
    "or" =>     TokenType::Or,
    "print" =>  TokenType::Print,
    "return" => TokenType::Return,
    "super" =>  TokenType::Super,
    "this" =>   TokenType::This,
    "true" =>   TokenType::True,
    "var" =>    TokenType::Var,
    "while" =>  TokenType::While,
};

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::scanner::tokenize;
    use crate::scanner::tokens::{LoxToken, TokenType};
    use crate::LoxResult;

    #[test]
    fn print() {
        let results = tokenize("print \"this is a test string\"");
        println!("{:?}", results);
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
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    line: 1,
                    column: 30,
                },
            ]
        )
    }

    // TODO whitespace ignored

    // TODO string with escape chars test

    // TODO tests detect an error but continue.
}
