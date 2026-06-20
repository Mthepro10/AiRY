use crate::interpreter::ast::{BinaryOp, Block, Expr, Program, Statement, UnaryOp, Value};
use crate::interpreter::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.pos - 1).unwrap()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.previous()
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn consume(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_delimiters(&mut self) {
        while matches!(self.peek(), Token::Newline | Token::Comma) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            self.skip_delimiters();
            if self.is_at_end() {
                break;
            }
            statements.push(self.statement()?);
        }

        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Set     => self.set_statement(),
            Token::Read    => self.read_statement(),
            Token::Show    => self.show_statement(),
            Token::If      => self.if_statement(),
            Token::Loop    => self.loop_statement(),
            Token::InfLoop => self.infloop_statement(),
            Token::Break   => { self.advance(); Ok(Statement::Break) }
            Token::Return  => { self.advance(); Ok(Statement::Return) }
            Token::Identifier(_) => self.assign_statement(),
            other => Err(format!("Unexpected token in statement: {:?}", other)),
        }
    }

    fn set_statement(&mut self) -> Result<Statement, String> {
        self.advance();

        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            other => return Err(format!("Expected variable name after 'set', got {:?}", other)),
        };

        if matches!(self.peek(), Token::Newline | Token::Comma | Token::EOF | Token::RBrace) {
            return Ok(Statement::Set { name, value: None });
        }

        let value = self.expr()?;
        Ok(Statement::Set { name, value: Some(value) })
    }

    fn read_statement(&mut self) -> Result<Statement, String> {
        self.advance();

        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            other => return Err(format!("Expected variable name after 'read', got {:?}", other)),
        };

        Ok(Statement::Read { name })
    }

    fn show_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'show'
        let value = self.expr()?;
        Ok(Statement::Show { value })
    }

    fn assign_statement(&mut self) -> Result<Statement, String> {
        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            other => return Err(format!("Expected identifier, got {:?}", other)),
        };

        if !self.consume(&Token::Assign) {
            return Err(format!("Expected '=' after '{}'", name));
        }

        let value = self.expr()?;
        Ok(Statement::Assign { name, value })
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'if'

        let condition = self.expr()?;
        let body = self.block()?;

        let mut elseif_branches = Vec::new();
        let mut else_body = None;

        loop {
            self.skip_delimiters();

            if self.consume(&Token::ElseIf) {
                let cond = self.expr()?;
                let block = self.block()?;
                elseif_branches.push((cond, block));
            } else if self.consume(&Token::Else) {
                else_body = Some(self.block()?);
                break;
            } else {
                break;
            }
        }

        Ok(Statement::If { condition, body, elseif_branches, else_body })
    }

    fn loop_statement(&mut self) -> Result<Statement, String> {
        self.advance();

        let variable = match self.advance().clone() {
            Token::Identifier(n) => n,
            other => return Err(format!("Expected variable name after 'loop', got {:?}", other)),
        };

        let start = self.primary()?;
        let end   = self.primary()?;
        let body  = self.block()?;

        Ok(Statement::Loop { variable, start, end, body })
    }

    fn infloop_statement(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.expr()?;
        let body = self.block()?;
        Ok(Statement::InfLoop { condition, body })
    }

    fn block(&mut self) -> Result<Block, String> {
        if !self.consume(&Token::LBrace) {
            return Err(format!("Expected '{{' to open block, got {:?}", self.peek()));
        }

        let mut statements = Vec::new();

        loop {
            self.skip_delimiters();

            if self.check(&Token::RBrace) || self.is_at_end() {
                break;
            }

            if matches!(self.peek(), Token::ElseIf | Token::Else) {
                break;
            }

            statements.push(self.statement()?);
        }

        if !self.consume(&Token::RBrace) {
            return Err(format!("Expected '}}' to close block, got {:?}", self.peek()));
        }

        Ok(statements)
    }

    fn expr(&mut self) -> Result<Expr, String> {
        if self.check(&Token::Perform) {
            return self.perform_expr();
        }
        self.or_expr()
    }

    fn perform_expr(&mut self) -> Result<Expr, String> {
        self.advance();
        let inner = self.or_expr()?;
        Ok(Expr::Perform(Box::new(inner)))
    }

    fn or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.and_expr()?;
        while self.consume(&Token::Or) {
            let right = self.and_expr()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::Or, right: Box::new(right) };
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.equality_expr()?;
        while self.consume(&Token::And) {
            let right = self.equality_expr()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::And, right: Box::new(right) };
        }
        Ok(left)
    }

    fn equality_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison_expr()?;
        loop {
            let op = match self.peek() {
                Token::Equal    => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.comparison_expr()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn comparison_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.bitor_expr()?;
        loop {
            let op = match self.peek() {
                Token::Greater      => BinaryOp::Greater,
                Token::Less         => BinaryOp::Less,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::LessEqual    => BinaryOp::LessEqual,
                _ => break,
            };
            self.advance();
            let right = self.bitor_expr()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn bitor_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.bitxor_expr()?;
        while self.consume(&Token::BitOr) {
            let right = self.bitxor_expr()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::BitOr, right: Box::new(right) };
        }
        Ok(left)
    }

    fn bitxor_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.bitand_expr()?;
        while self.consume(&Token::BitXor) {
            let right = self.bitand_expr()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::BitXor, right: Box::new(right) };
        }
        Ok(left)
    }

    fn bitand_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.additive_expr()?;
        while self.consume(&Token::BitAnd) {
            let right = self.additive_expr()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::BitAnd, right: Box::new(right) };
        }
        Ok(left)
    }

    fn additive_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.multiplicative_expr()?;
        loop {
            let op = match self.peek() {
                Token::Plus  => BinaryOp::Plus,
                Token::Minus => BinaryOp::Minus,
                _ => break,
            };
            self.advance();
            let right = self.multiplicative_expr()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn multiplicative_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.unary_expr()?;
        loop {
            let op = match self.peek() {
                Token::Star    => BinaryOp::Star,
                Token::Slash   => BinaryOp::Slash,
                Token::Percent => BinaryOp::Percent,
                _ => break,
            };
            self.advance();
            let right = self.unary_expr()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        Ok(left)
    }

    fn unary_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Minus  => { self.advance(); Ok(Expr::Unary { op: UnaryOp::Minus,  expr: Box::new(self.unary_expr()?) }) }
            Token::Not    => { self.advance(); Ok(Expr::Unary { op: UnaryOp::Not,    expr: Box::new(self.unary_expr()?) }) }
            Token::BitNot => { self.advance(); Ok(Expr::Unary { op: UnaryOp::BitNot, expr: Box::new(self.unary_expr()?) }) }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Integer(n)   => { self.advance(); Ok(Expr::Literal(Value::Integer(n))) }
            Token::Float(f)     => { self.advance(); Ok(Expr::Literal(Value::Float(f))) }
            Token::Boolean(b)   => { self.advance(); Ok(Expr::Literal(Value::Boolean(b))) }
            Token::StringLit(s) => { self.advance(); Ok(Expr::Literal(Value::String(s))) }
            Token::Identifier(name) => { self.advance(); Ok(Expr::Identifier(name)) }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.expr()?;
                if !self.consume(&Token::RParen) {
                    return Err(format!("Expected ')' after expression, got {:?}", self.peek()));
                }
                Ok(expr)
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }
}