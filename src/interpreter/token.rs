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
    Integer(i64),
    Float(f64),
    Boolean(bool),
    StringLit(String),

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
    Newline, 

    EOF,
}