use crate::error::LoxError;
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
                    return Err(LoxError::UnknownStringEscapeSequence(
                        lineno,
                        columnno,
                        format!("\\{}", next),
                    ));
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
    d >= '0' && d <= '9'
}
