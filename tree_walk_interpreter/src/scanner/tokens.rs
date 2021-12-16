#[derive(Clone, Debug)]
pub struct LoxToken {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl LoxToken {
    pub fn is_whitespace(&self) -> bool {
        match self.token_type {
            TokenType::Space | TokenType::Linefeed | TokenType::CarriageReturn | TokenType::Tab => {
                true
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    String(String),
    Number(f64),

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
