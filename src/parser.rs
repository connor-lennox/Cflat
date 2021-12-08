use crate::lexer::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    NUMBER(f64),
    VARIABLE(String),
    BINARY(Box<Expression>, char, Box<Expression>),
    FOR(String, Box<Expression>, Box<Expression>, Box<Expression>),
    IF(Box<Expression>, Box<Expression>)
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
        if let Token::CHARACTER(';') = self.get_next() {
            self.eat(); // Ignore top-level semicolons
        }
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expression {
        let lhs = self.parse_primary();
        self.parse_binary_op(1, lhs)
    }

    // Parse a "primary" token: number, identifier, or opening parenthesis
    fn parse_primary(&mut self) -> Expression {
        let exp = match self.get_next() {
            Token::IDENTIFIER(v) => Expression::VARIABLE(String::clone(v)), // No functions: all identifiers are going to be variables.
            Token::NUMBER(d) => Expression::NUMBER(*d),
            Token::CHARACTER('(') => self.parse_parens(),
            Token::FOR => todo!("FOR primary token"),
            Token::IF => todo!("IF primary token"),
            _ => panic!("invalid primary token {:?}", self.get_next())
        };

        self.eat(); // Eat the identifier, number, or closing parenthesis
        exp
    }

    fn parse_parens(&mut self) -> Expression {
        self.eat();                                 // Remove left parenthesis
        let lhs = self.parse_primary();   // Parse lhs of expression (might be only side)
        self.parse_binary_op(1, lhs)                   // Return a binary op with the pre-parsed lhs
    }

    fn parse_binary_op(&mut self, prec: u8, lhs: Expression) -> Expression {
        let mut comb = lhs;
        loop {
            // Get the precedence of the operation for this binary op
            let tok_prec = self.get_next_prec();

            if tok_prec < prec {
                return comb
            }

            let op: char;
            if let Token::CHARACTER(c) = self.get_next() {
                op = *c;
            } else {
                panic!("binary operation missing operator");
            }

            self.eat();     // Consume the operator
            let mut rhs = self.parse_primary();     // Get the rhs of the operator

            let next_prec = self.get_next_prec();     // Check to see if we forfeit this rhs to the next expression
            if tok_prec < next_prec {
                rhs = self.parse_binary_op(tok_prec + 1, rhs);
            }

            comb = Expression::BINARY(Box::<Expression>::new(comb), op, Box::<Expression>::new(rhs))
        }
    }

    // Get the next token (does not progress through tokens)
    fn get_next(&self) -> &Token {
        &self.input[self.idx]
    }

    // Get the precedence of the next token
    fn get_next_prec(&self) -> u8 {
        match self.get_next() {
            Token::CHARACTER(c) => token_precedence(*c),
            _ => 0
        }
    }

    // Eats a token, moving the pointer forward
    // Returns true when the final token is eaten
    fn eat(&mut self) -> bool {
        self.idx += 1;
        self.finished()
    }

    pub fn finished(&self) -> bool {
        self.idx == self.input.len()
    }
}

fn token_precedence(token: char) -> u8 {
    match token {
        '<' => 10,
        '+' => 20,
        '-' => 20,
        '*' => 40,
        '=' => 50,
        _ => 0
    }
}