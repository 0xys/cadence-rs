use std::fmt::Display;


#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    kind: NodeKind
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self { kind: kind }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    BinaryOperation(Box<Node>, Box<Node>, BinaryOperation),
    UnaryOperation(Box<Node>, Vec<UnaryOperation>),
    TerminalString(String),
    TerminalVariable(String),
    TerminalNumber(String),
    TerminalIdentifier(String),
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::BinaryOperation(lhs, rhs, op)
                => write!(f, "{}{{{}, {}}}", op, lhs.kind, rhs.kind),
            NodeKind::UnaryOperation(node, ops)
                => {
                    if ops.len() > 0 {
                        let mut s = String::from("[");
                        for i in 0..ops.len()-1 {
                            s.push_str(&format!("{}, ", &ops[i].to_string()))
                        }
                        s.push_str(&format!("{}", &ops[ops.len() - 1].to_string()));
                        s.push_str("]");
                        write!(f, "{}{{{}}}", s, node.kind)
                    } else {
                        write!(f, "{}", node)
                    }
                },
            NodeKind::TerminalString(str) => write!(f, "str({})", str),
            NodeKind::TerminalVariable(str) => write!(f, "var({})", str),
            NodeKind::TerminalNumber(str) => write!(f, "num({})", str),
            NodeKind::TerminalIdentifier(str) => write!(f, "id({})", str),
        }
    }
}

///
/// Shown according to operator precedences, higher to lower.
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperation {
    As,
    AsQuestion,
    AsExclamation,

    Mul,
    Div,
    Mod,

    Add,
    Sub,

    Shir,
    Shil,

    And,

    Xor,

    Or,

    NilCo,

    Lt,
    Gt,
    Lte,
    Gte,
    
    Eq,
    Neq,

    AndAnd,
    OrOr,
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Minus,
    Negate,
    Move,
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub value_type: FullType,
    pub value: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub is_resource: bool,
    pub full_type: FullType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FullType {
    Int,
    Int64,
    UInt,
    UInt64,
}