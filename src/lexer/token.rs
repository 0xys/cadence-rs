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
    ReverSlash, // \
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

    Digit,
    Alphabet,
    Symbol,

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

    EOF, // \u{0}
}