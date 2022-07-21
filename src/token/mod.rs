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

    FUNCTION,
    LET,
}

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
