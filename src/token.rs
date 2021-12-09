#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    EOF,
    True,
    False,
    If,
    For,
    Print,
    Comment,
    Dot,                    // .
    Comma,                  // ,
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
    Equals,                 // ==
    NotEquals,              // !=
    Not,                    // !
    LessThan,               // <
    GreaterThan,            // >
    LessEqual,              // <=
    GreaterEqual,           // >=
    Identifier(String),
    Number(f64),
    Unknown
}