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
    Dot, // .

    Plus, // +
    Minus, // -
    Asterisk, // *
    Slash, // /
    Backslash, // \
    Percent, // %

    Question,       // ?
    QuestionDot,    // ?.
    QuestionDouble, // ??
    Exclamation,    // !

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

    Swap,       // <->
    Move,       // <-
    MoveForce,  // <-!

    BitwiseAnd,
    BitwiseOr,
    BitwiseShiftRight,
    BitwiseShiftLeft,

    String(String),
    Identifier(String),
    Keyword(Keyword),

    None,

    EOF, // \u{0}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Var,

    True,
    False,

    Nil,

    If,
    Else,
    Switch,
    Case,
    Break,
    Default,
    While,
    For,
    In,
    Continue,

    Pub,
    Priv,
    Access,
    All,
    Contract,
    Account,

    Struct,
    Resource,
    Interface,
    Enum,
    Init,
    Get,
    Set,
    Pre,
    Post,
    SSelf,
    Create,
    Destroy,

    Import,
    From,

    Fun,
    Return,
    Event,
    Emit,

    Transaction,
    Prepare,
    Execute,

    As,
    AsEx,   // as!
    AsQu    // as?
}

impl Keyword {
    pub fn from(str: &str) -> Option<Self> {
        let keyword = match str {
            "let" => Self::Let,
            "var" => Self::Var,

            "true" => Self::True,
            "false" => Self::False,

            "nil" => Self::Nil,

            "if" => Self::If,
            "else" => Self::Else,
            "switch" => Self::Switch,
            "case" => Self::Case,
            "break" => Self::Break,
            "default" => Self::Default,
            "while" => Self::While,
            "for" => Self::For,
            "in" => Self::In,
            "continue" => Self::Continue,

            "pub" => Self::Pub,
            "priv" => Self::Priv,
            "access" => Self::Access,
            "all" => Self::All,
            "contract" => Self::Contract,
            "account" => Self::Account,

            "struct" => Self::Struct,
            "resource" => Self::Resource,
            "interface" => Self::Interface,
            "enum" => Self::Enum,
            "init" => Self::Init,
            "get" => Self::Get,
            "set" => Self::Set,
            "pre" => Self::Pre,
            "post" => Self::Post,
            "self" => Self::SSelf,
            "create" => Self::Create,
            "destroy" => Self::Destroy,

            "import" => Self::Import,
            "from" => Self::From,

            "fun" => Self::Fun,
            "return" => Self::Return,
            "event" => Self::Event,
            "emit" => Self::Emit,

            "transaction" => Self::Transaction,
            "prepare" => Self::Prepare,
            "execute" => Self::Execute,

            "as" => Self::As,
            _ => {
                return None
            }
        };
        Some(keyword)
    }
}