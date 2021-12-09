use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Boolean(bool),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Grouping(Box<Expression>),
    For(String, Box<Expression>, Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>)
}


pub struct Parser {
    input: Vec<Token>,
    idx: usize,
}


impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser{input, idx: 0}
    }

    // Parse all top-level expressions in the input
    pub fn parse(&mut self) -> Expression {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Expression {
        match self.get_next() {
            Token::If => self.parse_if(),
            Token::For => self.parse_for(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_if(&mut self) -> Expression {
        self.eat();     // Remove "if" token

        let condition = self.parse_expression();    // Get conditional statement
        assert!(self.consume_next() == &Token::OpenBrace, "missing open brace for if statement");
        
        let code = self.parse_statement();      // Get internals of if statement
        assert!(self.consume_next() == &Token::CloseBrace, "missing close brace for if statement");
        
        Expression::If(Box::<Expression>::new(condition), Box::<Expression>::new(code))
    }

    fn parse_for(&mut self) -> Expression {
        self.eat();     // Remove "for" token
        
        // Get the loop variable name
        let loop_var: String;
        if let Token::Identifier(v) = self.get_next() {
            loop_var = v.to_string();
            self.eat();
        } else {
            panic!("missing loop variable identifier");
        }
        assert!(self.consume_next() == &Token::Assignment, "missing assignment after for loop identifier");

        // Starting value for loop variable
        let start_expr: Expression = self.parse_expression();
        assert!(self.consume_next() == &Token::Colon, "missing separator for for loop");
        
        // Ending value for loop variable
        let end_expr: Expression = self.parse_expression();
        assert!(self.consume_next() == &Token::OpenBrace, "missing open brace for for loop");
        
        // Body of loop: the code that executes
        let body: Expression = self.parse_statement();
        assert!(self.consume_next() == &Token::CloseBrace, "missing close brace for for loop");

        Expression::For(loop_var, Box::<Expression>::new(start_expr), Box::<Expression>::new(end_expr), Box::<Expression>::new(body))
    }

    fn parse_expression_statement(&mut self) -> Expression {
        let expr = self.parse_expression();
        assert!(self.consume_next() == &Token::Semicolon, "missing semicolon on statement");
        expr
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Expression {
        let mut expr = self.parse_equality();

        if self.get_next() == &Token::Assignment {
            let op = self.consume_next().clone();
            let right = self.parse_equality();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_equality(&mut self) -> Expression {
        let mut expr = self.parse_comparison();

        while self.get_next() == &Token::NotEquals || self.get_next() == &Token::Equals {
            let op = self.consume_next().clone();
            let right = self.parse_comparison();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expression {
        let mut expr = self.parse_term();

        while self.get_next() == &Token::GreaterThan || self.get_next() == &Token::LessThan ||
                self.get_next() == &Token::GreaterEqual || self.get_next() == &Token::LessEqual {

            let op = self.consume_next().clone();
            let right = self.parse_term();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_term(&mut self) -> Expression {
        let mut expr = self.parse_factor();

        while self.get_next() == &Token::Plus || self.get_next() == &Token::Minus {
            let op = self.consume_next().clone();
            let right = self.parse_factor();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expression {
        let mut expr = self.parse_unary();

        while self.get_next() == &Token::Times || self.get_next() == &Token::Divide {
            let op = self.consume_next().clone();
            let right = self.parse_unary();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_unary(&mut self) -> Expression {
        if self.get_next() == &Token::Not || self.get_next() == &Token::Minus {
            let op = self.consume_next().clone();
            let right = self.parse_unary();
            return Expression::Unary(op, Box::new(right));
        }

        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Expression {
        match self.consume_next() {
            &Token::False => Expression::Boolean(false),
            &Token::True => Expression::Boolean(true),
            &Token::Number(v) => Expression::Number(v),
            Token::Identifier(v) => Expression::Variable(v.clone()),
            &Token::OpenParen => {
                let expr = self.parse_expression();
                self.eat();
                Expression::Grouping(Box::new(expr))
            }
            _ => panic!("invalid primary token type")
        }
    }

    // Get the next token (does not progress through tokens)
    fn get_next(&self) -> &Token {
        &self.input[self.idx]
    }

    fn consume_next(&mut self) -> &Token {
        self.idx += 1;
        &self.input[self.idx - 1]
    }

    // Eats a token, moving the pointer forward
    // Returns true when the final token is eaten
    fn eat(&mut self) -> bool {
        self.idx += 1;
        self.finished()
    }

    pub fn finished(&self) -> bool {
        self.get_next() == &Token::EOF
    }
}
