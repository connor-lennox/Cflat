use std::{fs::File, env, io::Read};

use token::Token;

mod token;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]);

    let mut file_buffer: Vec<u8> = Vec::<u8>::new();
    
    if let Ok(mut file) = f {
        match file.read_to_end(&mut file_buffer) {
            Ok(_) => (),
            Err(_) => panic!("could not read file"),
        }
    } else {
        panic!("could not find input file")
    };

    let char_buffer = file_buffer.iter().map(|b| *b as char).collect::<Vec<_>>();

    let mut lexer = lexer::Lexer::new(char_buffer);

    let mut tokens = Vec::<Token>::new();

    loop {
        let tok = lexer.get_token();
        println!("{:?}", tok);
        let done = tok == Token::EOF;
        tokens.push(tok);
        if done { break; }
    }

    tokens = tokens.into_iter().filter(|tok| *tok != Token::Comment).collect::<Vec<_>>();

    let mut tok_parser = parser::Parser::new(tokens);
    
    while !(tok_parser.finished()) {
        println!("{:?}", tok_parser.parse());
    }
}
