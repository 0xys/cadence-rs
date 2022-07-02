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
    Equal, // =

    Question, // ?
    Exclamation, // !

    At, // @

    And, // &
    Or, // |
    Xor, // ^
   
    DoubleQuote, // "
    SingleQuote, // '

    SemiColon, // ;
    Colon, // :

    Digit,
    Alphabet,
    Symbol,

    EOF, // \u{0}
}