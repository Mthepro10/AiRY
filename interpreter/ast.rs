#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    BitAnd,
    BitOr,
    BitXor,

    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,  // -x
    Not,    // !x
    BitNot, // ~x
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),

    Identifier(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Perform(Box<Expr>),
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Set {
        name: String,
        value: Option<Expr>,
    },

    Assign {
        name: String,
        value: Expr,
    },

    Read {
        name: String,
    },

    Show {
        value: Expr,
    },

    If {
        condition: Expr,
        body: Block,
        elseif_branches: Vec<(Expr, Block)>,
        else_body: Option<Block>,
    },

    Loop {
        variable: String,
        start: Expr,
        end: Expr,
        body: Block,
    },

    InfLoop {
        condition: Expr,
        body: Block,
    },

    Break,

    Return,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}