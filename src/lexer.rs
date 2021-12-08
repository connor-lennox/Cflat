use crate::token::Token;

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
            return Token::Comment;
        }

        // Check the single-character tokens
        match last_char {
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Divide,
            '=' => Token::Assignment,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            _ => Token::Unknown
        }
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
        }

        // Check keywords
        match id.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "for" => Token::For,
            "print" => Token::Print,
            _ => Token::Identifier(id)
        }
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

        Token::Number(num.parse::<f64>().unwrap())
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