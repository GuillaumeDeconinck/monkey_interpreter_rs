use crate::token::{Token, TokenType};

/// Lexer of the Monkey language
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

    fn peak_char(&self) -> Option<char> {
        if self.next_read_pos >= self.input.len() {
            None
        } else {
            Some(
                *self
                    .input
                    .get(self.next_read_pos)
                    .expect("Wrong read position"),
            )
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(current_char) = self.current_char {
            if current_char != ' '
                && current_char != '\t'
                && current_char != '\n'
                && current_char != '\r'
            {
                break;
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(current_char) = self.current_char {
            if Lexer::is_letter(current_char) {
                identifier.push(current_char);
                self.read_char();
            } else {
                break;
            }
        }

        identifier
    }

    fn read_number(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(current_char) = self.current_char {
            if Lexer::is_number(current_char) {
                identifier.push(current_char);
                self.read_char();
            } else {
                break;
            }
        }

        identifier
    }

    fn is_letter(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || (c == '_')
    }

    fn is_number(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn is_multi_chars_symbol(c: char, second_char_opt: Option<char>) -> bool {
        if let Some(second_char) = second_char_opt {
            if c == '!' && second_char == '=' {
                true
            } else {
                c == '=' && second_char == '='
            }
        } else {
            false
        }
    }

    // TODO: to convert to an actual Iterator
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let current_char = self.current_char?;

        let token_string = if Lexer::is_letter(current_char) {
            self.read_identifier()
        } else if Lexer::is_number(current_char) {
            self.read_number()
        } else if Lexer::is_multi_chars_symbol(current_char, self.peak_char()) {
            let mut string = String::from(current_char);
            self.read_char();
            let second_char = self.current_char?;
            self.read_char();
            string.push(second_char);
            string
        } else {
            let string = String::from(current_char);
            self.read_char();
            string
        };

        let token_type = TokenType::from(token_string.as_str());
        let token = Token {
            token_type,
            literal: current_char,
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_lexer_simple_chars() {
        let input = String::from("=+(){},;");

        struct TestCase(TokenType, char);

        let test_cases = [
            TestCase(TokenType::Assign, '='),
            TestCase(TokenType::Plus, '+'),
            TestCase(TokenType::Lparen, '('),
            TestCase(TokenType::Rparent, ')'),
            TestCase(TokenType::Lbrace, '{'),
            TestCase(TokenType::Rbrace, '}'),
            TestCase(TokenType::Comma, ','),
            TestCase(TokenType::Semicolon, ';'),
        ];

        let mut lexer = Lexer::new(input);

        for case in test_cases.into_iter() {
            let token = lexer.next_token().unwrap();

            println!("{:?}", &token);

            assert_eq!(token.token_type, case.0);
            assert_eq!(token.literal, case.1);
        }
    }

    #[test]
    fn test_lexer_real_1() {
        let input = String::from(
            "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);

        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ",
        );

        struct TestCase(TokenType, char);

        let test_cases = [
            TestCase(TokenType::Let, 'l'),
            TestCase(TokenType::Ident(String::from("five")), 'f'),
            TestCase(TokenType::Assign, '='),
            TestCase(TokenType::Int(5), '5'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Let, 'l'),
            TestCase(TokenType::Ident(String::from("ten")), 't'),
            TestCase(TokenType::Assign, '='),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Let, 'l'),
            TestCase(TokenType::Ident(String::from("add")), 'a'),
            TestCase(TokenType::Assign, '='),
            TestCase(TokenType::Function, 'f'),
            TestCase(TokenType::Lparen, '('),
            TestCase(TokenType::Ident(String::from("x")), 'x'),
            TestCase(TokenType::Comma, ','),
            TestCase(TokenType::Ident(String::from("y")), 'y'),
            TestCase(TokenType::Rparent, ')'),
            TestCase(TokenType::Lbrace, '{'),
            TestCase(TokenType::Ident(String::from("x")), 'x'),
            TestCase(TokenType::Plus, '+'),
            TestCase(TokenType::Ident(String::from("y")), 'y'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Rbrace, '}'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Let, 'l'),
            TestCase(TokenType::Ident(String::from("result")), 'r'),
            TestCase(TokenType::Assign, '='),
            TestCase(TokenType::Ident(String::from("add")), 'a'),
            TestCase(TokenType::Lparen, '('),
            TestCase(TokenType::Ident(String::from("five")), 'f'),
            TestCase(TokenType::Comma, ','),
            TestCase(TokenType::Ident(String::from("ten")), 't'),
            TestCase(TokenType::Rparent, ')'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Bang, '!'),
            TestCase(TokenType::Minus, '-'),
            TestCase(TokenType::Slash, '/'),
            TestCase(TokenType::Asterisk, '*'),
            TestCase(TokenType::Int(5), '5'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Int(5), '5'),
            TestCase(TokenType::Lt, '<'),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Gt, '>'),
            TestCase(TokenType::Int(5), '5'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::If, 'i'),
            TestCase(TokenType::Lparen, '('),
            TestCase(TokenType::Int(5), '5'),
            TestCase(TokenType::Lt, '<'),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Rparent, ')'),
            TestCase(TokenType::Lbrace, '{'),
            TestCase(TokenType::Return, 'r'),
            TestCase(TokenType::True, 't'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Rbrace, '}'),
            TestCase(TokenType::Else, 'e'),
            TestCase(TokenType::Lbrace, '{'),
            TestCase(TokenType::Return, 'r'),
            TestCase(TokenType::False, 'f'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Rbrace, '}'),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Eq, '='),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Semicolon, ';'),
            TestCase(TokenType::Int(10), '1'),
            TestCase(TokenType::Neq, '!'),
            TestCase(TokenType::Int(9), '9'),
            TestCase(TokenType::Semicolon, ';'),
        ];

        let mut lexer = Lexer::new(input);

        for case in test_cases.into_iter() {
            let token = lexer
                .next_token()
                .expect(format!("Got `None` while expecting the token {:?}", case.0).as_str());

            println!("{:?}", &token);

            assert_eq!(token.token_type, case.0);
            assert_eq!(token.literal, case.1);
        }

        // Lexer should return None once all the characters have been parsed
        assert_eq!(lexer.next_token().is_none(), true);
    }
}
