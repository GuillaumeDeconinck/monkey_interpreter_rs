use std::io::{BufRead, Write};

use crate::{lexer, token::TokenType};

const PROMPT: &str = ">> ";

pub fn start_repl(input: &mut dyn BufRead, output: &mut dyn Write) {
    loop {
        // TODO: handle errors more gracefully
        output.write_all(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut str_buffer = String::new();
        // TODO: handle errors more gracefully
        input.read_line(&mut str_buffer).unwrap();

        let mut lexer = lexer::Lexer::new(str_buffer);

        while let Some(token) = lexer.next_token() {
            if token.token_type == TokenType::Eof
                || token.token_type == TokenType::Ident("exit".into())
            {
                return;
            }

            // TODO: handle errors more gracefully
            output
                .write_all(format!("{:?}\n", token.token_type).as_bytes())
                .unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, BufWriter};

    use super::start_repl;

    #[test]
    fn test() {
        let input = String::from("let five = 5;exit");

        let mut buffer_read = BufReader::new(input.as_bytes());
        let mut buffer_write = BufWriter::new(vec![0; 50]);

        start_repl(&mut buffer_read, &mut buffer_write);

        let printed = String::from_utf8(buffer_write.buffer().to_owned()).unwrap();

        assert_eq!(printed, "Let\nIdent(\"five\")\nAssign\nInt(5)\nSemicolon\n");
    }
}
