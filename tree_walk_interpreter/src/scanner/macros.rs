#[macro_export]
macro_rules! consume_single_char_token {
    ($state:ident, $first:ident, $token_type:ident) => {{
        let (token, next_state) = consume_single_char_token($state, $first, TokenType::$token_type);
        (Ok(token), next_state)
    }};
}
