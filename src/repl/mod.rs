use std::io::{Stdin, Stdout, Write};

use crate::{lexer, token::TokenType};

const PROMPT: &str = ">> ";

pub fn start_repl(input: Stdin, mut output: Stdout) {
    loop {
        // TODO: handle errors more gracefully
        output.write_all(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut str_buffer = String::new();
        // TODO: handle errors more gracefully
        input.read_line(&mut str_buffer).unwrap();

        let mut lexer = lexer::Lexer::new(str_buffer);

        while let Some(token) = lexer.next_token() {
            if token.token_type == TokenType::Eof {
                return;
            }

            // TODO: handle errors more gracefully
            output
                .write_all(format!("{:?}\n", token.token_type).as_bytes())
                .unwrap();
        }
    }
}