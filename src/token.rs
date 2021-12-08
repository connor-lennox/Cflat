#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    EOF,
    True,
    False,
    If,
    For,
    Print,
    Comment,
    Colon,                  // :
    Semicolon,              // ;
    OpenParen,              // (
    CloseParen,             // )
    OpenBracket,            // [
    CloseBracket,           // ]
    OpenBrace,              // {
    CloseBrace,             // }
    Plus,                   // +
    Minus,                  // -
    Times,                  // *
    Divide,                 // /
    Assignment,             // =
    LessThan,               // <
    GreaterThan,            // >
    Identifier(String),
    Number(f64),
    Unknown
}