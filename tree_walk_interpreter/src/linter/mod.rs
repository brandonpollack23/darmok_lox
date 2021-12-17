use std::fs::read_to_string;

use crate::{scan, LinterError, LoxError, LoxResult, LoxToken, TokenType};

#[cfg(test)]
mod tests;

// TODO linting mode: multiple steps, first just reads token streams for token rules (double space not after newline, use of tabs, etc)
// > handle tab (warning no tabs use spaces)
// > handle non newline whitespace with more than one space (linter warning)
// > warn there should be no \r\n on linux
pub fn lint(file_name: &str) -> anyhow::Result<Vec<LoxError>> {
    let script = read_to_string(file_name)?;
    let tokens = scan(&script);

    // This obviously not complete, just serves as an example
    let tokenizer_lint_errors = lint_tokens(&tokens);

    // let parser_lint_errors = lint(parse(tokens))?;
    // then print all

    // Append all types of lint errors
    Ok(tokenizer_lint_errors)
}

pub fn lint_tokens(tokens: &[LoxResult<LoxToken>]) -> Vec<LoxError> {
    // Finds double spaces.  Wouldn't even really work because it doesn't detect if they're on a blank line or more than two.
    tokens
        .windows(2)
        .filter(|w| {
            matches!(
                w,
                [
                    Ok(LoxToken {
                        token_type: TokenType::Space,
                        ..
                    }),
                    Ok(LoxToken {
                        token_type: TokenType::Space,
                        ..
                    })
                ]
            )
        })
        .map(|w| {
            let first_space = w.get(0).cloned().unwrap().unwrap();
            LinterError::DoubleSpaceDetected(first_space.line, first_space.column).into()
        })
        .collect()
}
