use crate::{lexer::Lexer, token::Token};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    LessGreater,
    Sum,
    Product,
}

#[derive(Clone)]
enum Node {
    TokenNode(Token),
    InfixNode {
        value: Token,
        left: Box<Option<Self>>,
        right: Box<Option<Self>>,
    },
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            peek_token: Token::Illegal,
            current_token: Token::Illegal,
        };

        // Set current token and peek token.
        parser.lexer.next_token();
        parser.lexer.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_expression(&mut self, precedence: &Precedence) -> Node {
        let left = self.parse_tree_node(self.current_token);

        while precedence < &self.get_token_precedence(&self.peek_token) {
            let infix = &self.parse_infix_expression(left);
        }

        Node::TokenNode(Token::EOF)
    }

    fn parse_tree_node(&self, token: Token) -> Node {
        match token {
            Token::Int(_) => Node::TokenNode(token),
            Token::LT | Token::GT | Token::Plus | Token::Minus | Token::Slash | Token::Asterisk => {
                Node::InfixNode {
                    value: token,
                    left: Box::new(None),
                    right: Box::new(None),
                }
            }
            _ => Node::TokenNode(token),
        }
    }

    fn get_token_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Int(_) => Precedence::Lowest,
            Token::LT => Precedence::LessGreater,
            Token::GT => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn parse_infix_expression(&mut self, left: Node) -> Node {
        let precedence = self.get_token_precedence(&self.peek_token);

        self.next_token();

        let right = self.parse_expression(&precedence);

        Node::InfixNode {
            value: self.current_token,
            left: Box::from(Some(left)),
            right: Box::new(Some(right)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Precedence};

    use super::Parser;

    #[test]
    fn it_correctly_parses_tokens_simple() {
        let mut parser = Parser::new(Lexer::new("5 * 5"));

        let expr = parser.parse_expression(&Precedence::Lowest);

        assert_eq!(expr, 25);
    }
}
