/// The different types of tokens in the Monkey language
#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(i64),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Incomplete
    FUNCTION,
    LET,
}

impl From<char> for TokenType {
    fn from(to_parse: char) -> Self {
        match to_parse {
            '=' => TokenType::ASSIGN,
            '+' => TokenType::PLUS,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            // TODO: to complete
            _ => TokenType::ILLEGAL,
        }
    }
}

/// A specific token
///
/// Not sure it will stays in the long run
pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl Token {
    pub fn new(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
