use crate::token::{Token, TokenType};

struct Lexer {
    input: Vec<char>,
    current_pos: usize,
    next_read_pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            current_pos: 0,
            current_char: None,
            next_read_pos: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.next_read_pos >= self.input.len() {
            self.current_char = None
        } else {
            self.current_char = Some(
                *self
                    .input
                    .get(self.next_read_pos)
                    .expect("Wrong read position"),
            );
        }
        self.current_pos = self.next_read_pos;
        self.next_read_pos += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let current_char = self.current_char?;

        let token_type = match current_char {
            '=' => TokenType::ASSIGN,
            '+' => TokenType::PLUS,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            _ => TokenType::ILLEGAL,
        };
        let token = Token {
            token_type,
            literal: current_char,
        };

        self.read_char();
        Some(token)
    }
}

#[test]
fn test_lexer() {
    let input = String::from("=+(){{}},;");

    struct TestCase(TokenType, char);

    let test_cases = [
        TestCase(TokenType::ASSIGN, '='),
        TestCase(TokenType::PLUS, '+'),
        TestCase(TokenType::LPAREN, '('),
        TestCase(TokenType::RPAREN, ')'),
    ];

    let mut lexer = Lexer::new(input);

    for case in test_cases.into_iter() {
        let token = lexer.next_token().unwrap();

        assert_eq!(token.token_type, case.0);
        assert_eq!(token.literal, case.1);
    }
}
