use crate::error::{LoxError, ScannerError};
use crate::LoxResult;

// BONUS: unicode values \u
/// Unescapes a string for \, ", \n, and \t
pub fn unescape_string(string: &str, lineno: usize, columno: usize) -> LoxResult<String> {
    let mut result = String::with_capacity(string.len());
    let mut lineno = lineno;
    let mut columnno = columno;
    for (i, c) in string.chars().enumerate() {
        if c == '\\' {
            let next = string.chars().nth(i).unwrap_or('\\');
            match next {
                '\\' => result.push(next),
                '"' => result.push('"'),
                'n' => result.push('\n'),
                't' => result.push('\t'),
                _ => {
                    return Err(ScannerError::UnknownStringEscapeSequence(
                        lineno,
                        columnno,
                        format!("\\{}", next),
                    )
                    .into());
                }
            }
        } else {
            result.push(c)
        }

        if c == '\n' {
            lineno += 1;
            columnno = 1;
        }
    }

    Ok(result)
}

pub fn is_digit(d: char) -> bool {
    ('0'..='9').contains(&d)
}

pub fn is_alpha(c: char) -> bool {
    ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || (c == '_')
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}
