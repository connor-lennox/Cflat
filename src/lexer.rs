#[derive(PartialEq, Debug)]
pub enum Token {
    EOF,
    IF,
    FOR,
    COMMENT,
    IDENTIFIER(String),
    NUMBER(f64),
    CHARACTER(char)
}

pub struct Lexer {
    input: Vec<char>,
    idx: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer {input, idx: 0} 
    }
    
    pub fn get_token(&mut self) -> Token {
        let mut last_char: char;

        match self.get_char() {
            Some(c) => {
                last_char = c;
                self.progress();
            },
            None => return Token::EOF,
        }

        while last_char.is_whitespace() {
            match self.get_char() {
                Some(c) => {
                    last_char = c;
                    self.progress();
                },
                None => return Token::EOF,
            }
        }

        // Identifiers start with a letter
        if last_char.is_alphabetic() {
            return self.get_identifier(last_char);
        }

        // Numbers - everything is an f64
        if last_char.is_numeric() {
            return self.get_number(last_char);
        }

        // Comments
        if last_char == '#' {
            while last_char != '\n' && last_char != '\r' {
                match self.get_char() {
                    Some(c) => {
                        last_char = c;
                        self.progress();
                    },
                    None => return Token::EOF,
                }
            }
            return Token::COMMENT;
        }

        // If nothing else worked, just pass back the token
        return Token::CHARACTER(last_char);
    }

    fn get_identifier(&mut self, starting_char: char) -> Token {
        let mut id = String::from(starting_char);
        let mut c = self.get_char();

        // while let Some(next_char) = c && next_char.is_alphanumeric() {
        while let Some(next_char) = c {
            if !(next_char.is_alphanumeric()) { break; }

            id.push(next_char);
            self.progress();
            c = self.get_char();

            if id.eq("if") { return Token::IF; }
            if id.eq("for") { return Token::FOR; }
        }

        return Token::IDENTIFIER(id);
    }

    fn get_number(&mut self, starting_char: char) -> Token {
        let mut num = String::from(starting_char);
        let mut c = self.get_char();

        while let Some(next_char) = c {
            if !(next_char.is_numeric() || next_char == '.') { break; }

            num.push(next_char);
            self.progress();
            c = self.get_char();
        }

        Token::NUMBER(num.parse::<f64>().unwrap())
    }

    fn get_char(&mut self) -> Option<char> {
        if self.idx < self.input.len() {
            Some(self.input[self.idx])
        } else {
            None
        }
    }

    fn progress(&mut self) {
        self.idx += 1;
    }
}