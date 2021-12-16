use crate::utils::unescape_string;
use crate::LoxResult;

pub trait UnEscapableString {
    fn unescape_string(&self, lineno: usize, columno: usize) -> LoxResult<String>;
}

impl UnEscapableString for str {
    fn unescape_string(&self, lineno: usize, columno: usize) -> LoxResult<String> {
        unescape_string(self, lineno, columno)
    }
}
