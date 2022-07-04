#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub at: usize,
    pub size: usize,
}

impl Token {
    pub fn new(kind: TokenKind, at: usize, size: usize) -> Self {
        Token { kind, at, size }
    }

    pub fn new_c(kind: TokenKind, at: usize) -> Self {
        Token { kind, at, size: 1 }
    }

    pub fn new_none(at: usize, size: usize) -> Self {
        Token { kind: TokenKind::None, at, size }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    ParenOpen, // (
    ParenClose, // )
    BracketOpen, // [
    BracketClose, // ]
    BraceOpen, // {
    BraceClose, // }
    AngleOpen, // <
    AngleClose, // >
    
    Comma, // ,
    Period, // .

    Plus, // +
    Minus, // -
    Asterisk, // *
    Slash, // /
    Backslash, // \
    Percent, // %

    Question, // ?
    Exclamation, // !

    At, // @

    LogicalConjunction, // &
    LogicalDisjunction, // |
    Xor, // ^
   
    DoubleQuote, // "
    SingleQuote, // '

    SemiColon, // ;
    Colon, // :

    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    Assign,
    EqualTo,

    BitwiseAnd,
    BitwiseOr,
    BitwiseShiftRight,
    BitwiseShiftLeft,

    String(String),
    Identifier(String),

    As,

    None,
    LineComment,

    EOF, // \u{0}
}