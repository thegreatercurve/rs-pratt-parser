#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Int(usize),
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Eq,
    NotEq,
    LParen,
    RParen,
}
