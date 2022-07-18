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

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Any,
    AnyStruct,
    AnyResource,

    Never,

    Address,
    Character,
    String,
    Bool,
    Type,

    Number,
    FixedPoint,
    SignedNumber,
    UFix64,
    SignedFixedPoint,
    Fix64,

    Integer,
    UInt,
    Uint8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    UInt256,
    Word8,
    Word16,
    Word32,
    Word64,
    
    SignedInteger,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,

    Block,
    Capability,
    Reference,

    Path,
    StoragePath,
    CapabilityPath,
    PublicPath,
    PrivatePath,

    Function,
    AuthAccount,
    PublicAccount,
    Enum,

    ArrayOfStructs,
    DictionaryOfStructs,
    OptionalOfStructs,
    RestrictionOfStructs,

    ArrayOfResources,
    DictionaryOfResources,
    OptionalOfResources,
    RestrictionOfResources,
}

impl Type {
    pub fn from(str: &str) -> Option<Self> {
        let _type = match str {
            "Any" => Self::Any,
            "AnyStruct" => Self::AnyStruct,
            "AnyResource" => Self::AnyResource,

            "Never" => Self::Never,

            "Address" => Self::Address,
            "Character" => Self::Character,
            "String" => Self::String,
            "Bool" => Self::Bool,
            "Type" => Self::Type,

            "Number" => Self::Number,
            "FixedPoint" => Self::FixedPoint,
            "SignedNumber" => Self::SignedNumber,
            "UFix64" => Self::UFix64,
            "SignedFixedPoint" => Self::SignedFixedPoint,
            "Fix64" => Self::Fix64,

            "Integer" => Self::Integer,
            "UInt" => Self::UInt,
            "Uint8" => Self::Uint8,
            "UInt16" => Self::UInt16,
            "UInt32" => Self::UInt32,
            "UInt64" => Self::UInt64,
            "UInt128" => Self::UInt128,
            "UInt256" => Self::UInt256,
            "Word8" => Self::Word8,
            "Word16" => Self::Word16,
            "Word32" => Self::Word32,
            "Word64" => Self::Word64,

            "SignedInteger" => Self::SignedInteger,
            "Int" => Self::Int,
            "Int8" => Self::Int8,
            "Int16" => Self::Int16,
            "Int32" => Self::Int32,
            "Int64" => Self::Int64,
            "Int128" => Self::Int128,
            "Int256" => Self::Int256,

            "Block" => Self::Block,
            "Capability" => Self::Capability,
            "Reference" => Self::Reference,

            "Path" => Self::Path,
            "StoragePath" => Self::StoragePath,
            "CapabilityPath" => Self::CapabilityPath,
            "PublicPath" => Self::PublicPath,
            "PrivatePath" => Self::PrivatePath,

            "Function" => Self::Function,
            "AuthAccount" => Self::AuthAccount,
            "PublicAccount" => Self::PublicAccount,
            "Enum" => Self::Enum,

            "ArrayOfStructs" => Self::ArrayOfStructs,
            "DictionaryOfStructs" => Self::DictionaryOfStructs,
            "OptionalOfStructs" => Self::OptionalOfStructs,
            "RestrictionOfStructs" => Self::RestrictionOfStructs,

            "ArrayOfResources" => Self::ArrayOfResources,
            "DictionaryOfResources" => Self::DictionaryOfResources,
            "OptionalOfResources" => Self::OptionalOfResources,
            "RestrictionOfResources" => Self::RestrictionOfResources,

            _ => {
                return None
            }
        };

        Some(_type)
    }
}