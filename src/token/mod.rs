use once_cell::sync::Lazy;
use regex::Regex;

/// The different types of tokens in the Monkey language
#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident(String),
    Int(i32),

    Assign,
    Plus,
    Minus,
    Eq,
    Neq,

    Comma,
    Semicolon,
    Bang,
    Asterisk,
    Slash,

    Lparen,
    Rparent,
    Lbrace,
    Rbrace,
    Lt,
    Gt,

    // Incomplete
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

impl From<&str> for TokenType {
    fn from(to_parse: &str) -> Self {
        match to_parse {
            "=" => TokenType::Assign,
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "==" => TokenType::Eq,
            "!=" => TokenType::Neq,
            "(" => TokenType::Lparen,
            ")" => TokenType::Rparent,
            "{" => TokenType::Lbrace,
            "}" => TokenType::Rbrace,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "!" => TokenType::Bang,
            "*" => TokenType::Asterisk,
            "/" => TokenType::Slash,
            "<" => TokenType::Lt,
            ">" => TokenType::Gt,
            "" => TokenType::Eof,
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ if REGEX_IDENTIFIER.is_match(to_parse) => TokenType::Ident(to_parse.into()),
            _ if REGEX_INT.is_match(to_parse) => TokenType::Int(str::parse(to_parse).unwrap()),
            _ => TokenType::Illegal,
        }
    }
}

static REGEX_IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new("[a-zA-Z_]+").unwrap());
static REGEX_INT: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9]+").unwrap());

/// A specific token
///
/// Not sure it will stays in the long run
#[derive(Debug)]
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
