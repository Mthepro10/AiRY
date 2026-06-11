use crate::interpreter::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn lex_number(&mut self) -> Result<Token, String> {
        let start = self.pos;
        let mut has_dot = false;

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.advance();
                }
                '.' if !has_dot => {
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();

        if has_dot {
            text.parse::<f64>()
                .map(Token::Float)
                .map_err(|_| format!("Invalid float literal: {}", text))
        } else {
            text.parse::<i64>()
                .map(Token::Integer)
                .map_err(|_| format!("Invalid integer literal: {}", text))
        }
    }

    fn lex_string(&mut self) -> Result<Token, String> {
        self.advance();

        let start = self.pos;
        while let Some(c) = self.peek() {
            if c == '"' {
                let text: String = self.chars[start..self.pos].iter().collect();
                self.advance();
                return Ok(Token::StringLit(text));
            }
            self.advance();
        }

        Err("Unterminated string literal".to_string())
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    self.advance();
                }
                _ => break,
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();

        match text.as_str() {
            "read" => Token::Read,
            "show" => Token::Show,
            "set" => Token::Set,
            "perform" => Token::Perform,

            "if" => Token::If,
            "elseif" => Token::ElseIf,
            "else" => Token::Else,

            "loop" => Token::Loop,
            "infloop" => Token::InfLoop,

            "break" => Token::Break,
            "return" => Token::Return,

            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),

            _ => Token::Identifier(text),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_spaces();

            if self.is_at_end() {
                break;
            }

            let c = match self.peek() {
                Some(ch) => ch,
                None => break,
            };

            let token = match c {
                '\n' => {
                    self.advance();
                    Token::Newline
                }

                ',' => {
                    self.advance();
                    Token::Comma
                }

                '(' => {
                    self.advance();
                    Token::LParen
                }

                ')' => {
                    self.advance();
                    Token::RParen
                }

                '{' => {
                    self.advance();
                    Token::LBrace
                }

                '}' => {
                    self.advance();
                    Token::RBrace
                }

                '+' => {
                    self.advance();
                    Token::Plus
                }

                '-' => {
                    self.advance();
                    Token::Minus
                }

                '*' => {
                    self.advance();
                    Token::Star
                }

                '/' => {
                    self.advance();
                    Token::Slash
                }

                '%' => {
                    self.advance();
                    Token::Percent
                }

                '^' => {
                    self.advance();
                    Token::BitXor
                }

                '~' => {
                    self.advance();
                    Token::BitNot
                }

                '=' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.advance();
                        Token::Equal
                    } else {
                        self.advance();
                        Token::Assign
                    }
                }

                '!' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.advance();
                        Token::NotEqual
                    } else {
                        self.advance();
                        Token::Not
                    }
                }

                '<' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.advance();
                        Token::LessEqual
                    } else {
                        self.advance();
                        Token::Less
                    }
                }

                '>' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.advance();
                        Token::GreaterEqual
                    } else {
                        self.advance();
                        Token::Greater
                    }
                }

                '&' => {
                    if self.peek_next() == Some('&') {
                        self.advance();
                        self.advance();
                        Token::And
                    } else {
                        self.advance();
                        Token::BitAnd
                    }
                }

                '|' => {
                    if self.peek_next() == Some('|') {
                        self.advance();
                        self.advance();
                        Token::Or
                    } else {
                        self.advance();
                        Token::BitOr
                    }
                }

                '"' => self.lex_string()?,
                '0'..='9' => self.lex_number()?,
                'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(),

                other => {
                    return Err(format!("Unexpected character: {}", other));
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }
}