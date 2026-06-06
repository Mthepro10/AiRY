#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Read,
    Show,
    Set,
    Perform,

    If,
    ElseIf,
    Else,

    Loop,
    InfLoop,

    Break,
    Return,

    Identifier(String),

    Number(i64),
    Float(f64),

    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    Assign,

    Equal,
    NotEqual,

    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    And,
    Or,
    Not,

    LParen,
    RParen,

    LBrace,
    RBrace,

    Comma,

    EOF,
}