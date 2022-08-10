use std::fmt::Display;


#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    node: ConditionalExpression
}
impl Expression {
    pub fn new(node: ConditionalExpression) -> Self {
        Self {
            node,
        }
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!("{}", self.node)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalExpression {
    exp: OrExpression
}
impl ConditionalExpression {
    pub fn new(exp: OrExpression) -> Self {
        Self {
            exp,
        }
    }
}
impl Display for ConditionalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!("{}", self.exp)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrExpression {
    left: Box<OrExpression>,
    right: AndExpression,
}
impl OrExpression {
    pub fn new(left: OrExpression, right: AndExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AndExpression {
    left: Box<AndExpression>,
    right: EqualExpression,
}
impl AndExpression {
    pub fn new(left: AndExpression, right: EqualExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EqualExpression {
    left: Box<EqualExpression>,
    right: RelationExpression,
}
impl EqualExpression {
    pub fn new(left: EqualExpression, right: RelationExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationExpression {
    left: Box<RelationExpression>,
    right: NilCoalescingExpression,
}
impl RelationExpression {
    pub fn new(left: RelationExpression, right: NilCoalescingExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NilCoalescingExpression {
    left: Box<NilCoalescingExpression>,
    right: BitwiseOrExpression,
}
impl NilCoalescingExpression {
    pub fn new(left: NilCoalescingExpression, right: BitwiseOrExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitwiseOrExpression {
    left: Box<BitwiseOrExpression>,
    right: BitwiseXorExpression,
}
impl BitwiseOrExpression {
    pub fn new(left: BitwiseOrExpression, right: BitwiseXorExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitwiseXorExpression {
    left: Box<BitwiseXorExpression>,
    right: BitwiseAndExpression,
}
impl BitwiseXorExpression {
    pub fn new(left: BitwiseXorExpression, right: BitwiseAndExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitwiseAndExpression {
    left: Box<BitwiseAndExpression>,
    right: BitwiseShiftExpression,
}
impl BitwiseAndExpression {
    pub fn new(left: BitwiseAndExpression, right: BitwiseShiftExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitwiseShiftExpression {
    left: Box<BitwiseShiftExpression>,
    right: AdditiveExpression,
}
impl BitwiseShiftExpression {
    pub fn new(left: BitwiseAndExpression, right: AdditiveExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdditiveExpression {
    left: Box<AdditiveExpression>,
    right: MultiplicativeExpression,
}
impl AdditiveExpression {
    pub fn new(left: AdditiveExpression, right: MultiplicativeExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultiplicativeExpression {
    left: Box<MultiplicativeExpression>,
    right: CastingExpression,
}
impl MultiplicativeExpression {
    pub fn new(left: MultiplicativeExpression, right: CastingExpression) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CastingExpression {
    left: UnaryExpression,
    right: TypeAnnotation,
}
impl CastingExpression {
    pub fn new(left: UnaryExpression, right: TypeAnnotation) -> Self {
        Self {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    left: PrimaryExpression,
    right: Box<UnaryExpression>,
}
impl UnaryExpression {
    pub fn new(left: PrimaryExpression, right: UnaryExpression) -> Self {
        Self {
            left,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryExpression {
    Create,
    Destroy,
    Reference(Box<ReferenceExpression>),
    Postfix(Box<PostfixExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PostfixExpression {
    Identifier(String),
    Literal(String),
    Fun(String),
    Factor(Box<Expression>), // '(' expression ')'
    Exclaim(Box<PostfixExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceExpression {
    exp: Box<Expression>,
    full_type: FullType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionAccess {
    MemberAcceess(Box<MemberAccess>),
    BracketExpression(Box<BracketExpression>),
}
impl Display for ExpressionAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MemberAcceess(str) => write!(f, "Type{{auth &{}}}", str),
            Self::BracketExpression(str) => write!(f, "Type{{&{}}}", str),
            Self::Inner(str) => write!(f, "Type{{{}}}", str),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccess {
    optional: bool,
    exp: Box<BracketExpression>,
}
impl MemberAccess {
    pub fn new(optional: bool, exp: BracketExpression) -> Self {
        Self {
            optional,
            exp: Box::new(exp),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BracketExpression {
    exp: Box<Expression>,
}
impl BracketExpression {
    pub fn new(exp: Expression) -> Self {
        Self {
            exp: Box::new(exp),
        }
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
pub struct Argument {
    pub id: Option<String>,
    pub exp: Box<Expression>,
}
impl Argument {
    pub fn new(id: Option<String>, exp: Expression) -> Self {
        Self { id, exp: Box::new(exp) }
    }
}
impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_str = match self.id {
            Some(id) => format!("{}: ", id),
            None => "".to_string(),
        };
        write!(f, "Arg{{{}{}}}", id_str, self.exp)
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