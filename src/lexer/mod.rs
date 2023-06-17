use crate::token::Token;

pub struct Lexer {
    input: Box<Vec<char>>,
    current_index: usize,
    peek_index: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars = Box::from(input.chars().collect::<Vec<char>>());

        let mut lexer = Lexer {
            input: chars,
            current_index: 0,
            peek_index: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.peek_index >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.peek_index();
        }

        self.current_index = self.peek_index;
        self.peek_index += 1
    }

    fn peek_index(&self) -> Option<char> {
        if self.peek_index >= self.input.len() {
            None
        } else {
            Some(self.input[self.peek_index])
        }
    }

    #[allow(dead_code)]
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token = Token::Illegal;

        match &self.ch {
            Some('=') => match self.peek_index() {
                Some('=') => {
                    self.read_char();

                    token = Token::Eq
                }
                _ => token = Token::Illegal,
            },
            Some('+') => token = Token::Plus,
            Some('-') => token = Token::Minus,
            Some('!') => match self.peek_index() {
                Some('=') => {
                    self.read_char();

                    token = Token::NotEq
                }
                _ => token = Token::Illegal,
            },
            Some('/') => token = Token::Slash,
            Some('*') => token = Token::Asterisk,
            Some('<') => token = Token::LT,
            Some('>') => token = Token::GT,
            Some('(') => token = Token::LParen,
            Some(')') => token = Token::RParen,
            Some(ch) => {
                if ch.is_numeric() {
                    let num_literal = self.read_number();

                    token = Token::Int(num_literal);

                    return token;
                } else {
                    token = Token::Illegal
                }
            }
            None => token = Token::EOF,
        };

        self.read_char();

        token
    }

    fn read_number(&mut self) -> usize {
        let index = self.current_index;

        while self.ch.unwrap_or_default().is_numeric() {
            self.read_char();
        }

        let char_vec = &self.input[index..self.current_index];

        char_vec
            .iter()
            .cloned()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ') {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn it_correctly_parses_tokens_simple() {
        let mut lexer = Lexer::new("5 * 5");

        let tokens = vec![Token::Int(5), Token::Asterisk, Token::Int(5)];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn it_correctly_parses_tokens_multiple_no_spaces() {
        let mut lexer = Lexer::new("3+9/4");

        let tokens = vec![
            Token::Int(3),
            Token::Plus,
            Token::Int(9),
            Token::Slash,
            Token::Int(4),
        ];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn it_correctly_parses_tokens_multiple_digit_numbers() {
        let mut lexer = Lexer::new("(86915 / 2)");

        let tokens = vec![
            Token::LParen,
            Token::Int(86915),
            Token::Slash,
            Token::Int(2),
            Token::RParen,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn it_correctly_parses_tokens_with_braces() {
        let mut lexer = Lexer::new("((12 * 3242) + 5)");

        let tokens = vec![
            Token::LParen,
            Token::LParen,
            Token::Int(12),
            Token::Asterisk,
            Token::Int(3242),
            Token::RParen,
            Token::Plus,
            Token::Int(5),
            Token::RParen,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn it_correctly_parses_tokens_with_equality_check() {
        let mut lexer = Lexer::new("5 * 5 - 2 == 23");

        let tokens = vec![
            Token::Int(5),
            Token::Asterisk,
            Token::Int(5),
            Token::Minus,
            Token::Int(2),
            Token::Eq,
            Token::Int(23),
        ];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn it_correctly_parses_tokens_with_non_equality_check() {
        let mut lexer = Lexer::new("5 * 12332 - 2 != 22");

        let tokens = vec![
            Token::Int(5),
            Token::Asterisk,
            Token::Int(12332),
            Token::Minus,
            Token::Int(2),
            Token::NotEq,
            Token::Int(22),
        ];

        for expected_token in tokens {
            let token = lexer.next_token();

            assert_eq!(expected_token, token);
        }
    }
}
