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
    Destroy(Box<Node>),
    TerminalString(String),
    TerminalVariable(String),
    TerminalNumber(String),
    TerminalIdentifier(String),
    ResourceTypeAnnotation(FullType),
    TypeAnnotation(FullType),
    // Invocation(Option<Vec<Node>>, Vec<Node>),
    Argument(ArgumentExp),
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::BinaryOperation(lhs, rhs, op)
                => write!(f, "{}{{{}, {}}}", op, lhs, rhs),
            NodeKind::UnaryOperation(node, ops)
                => {
                    if ops.len() > 0 {
                        let mut s = String::from("[");
                        for i in 0..ops.len()-1 {
                            s.push_str(&format!("{}, ", &ops[i].to_string()))
                        }
                        s.push_str(&format!("{}", &ops[ops.len() - 1].to_string()));
                        s.push_str("]");
                        write!(f, "{}{{{}}}", s, node)
                    } else {
                        write!(f, "{}", node)
                    }
                },
            NodeKind::Destroy(node) => write!(f, "Destroy{{{}}}", node),
            NodeKind::TerminalString(str) => write!(f, "str({})", str),
            NodeKind::TerminalVariable(str) => write!(f, "var({})", str),
            NodeKind::TerminalNumber(str) => write!(f, "num({})", str),
            NodeKind::TerminalIdentifier(str) => write!(f, "id({})", str),

            NodeKind::ResourceTypeAnnotation(t) => write!(f, "@{}", t),
            NodeKind::TypeAnnotation(t) => write!(f, "{}", t),

            NodeKind::Argument(arg) => {
                let mut id = String::new();
                if let Some(id_) = arg.id.clone() {
                    id.push_str(&id_);
                    id.push_str(": ")
                }
                write!(f, "Arg{{{}{}}}", id, arg.exp)
            },
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
pub enum FullType {
    AuthReference(String),
    Reference(String),
    Inner(String),
}
impl Display for FullType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthReference(str) => write!(f, "Type{{auth &{}}}", str),
            Self::Reference(str) => write!(f, "Type{{&{}}}", str),
            Self::Inner(str) => write!(f, "Type{{{}}}", str),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentExp {
    pub id: Option<String>,
    pub exp: Box<Node>,
}
impl ArgumentExp {
    pub fn new(id: Option<String>, exp: Node) -> Self {
        Self { id: id, exp: Box::new(exp) }
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