
pub trait AstNode<T> {
    fn eval(&self) -> T;
}

pub struct Expression {

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

#[derive(Clone, Debug, PartialEq)]
pub enum AdditiveExpression {
    Mult(Box<MultiplicativeExpression>),
    Add(Box<InnerAdditiveExpression>)
}
impl AstNode<usize> for AdditiveExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Mult(e) => {
                e.eval()
            },
            Self::Add(e) => {
                match &e.op {
                    AdditiveOperator::Plus => {
                        let lhs = e.exp.eval();
                        let rhs = e.mult_exp.eval();
                        lhs + rhs
                    },
                    AdditiveOperator::Minus => {
                        let lhs = e.exp.eval();
                        let rhs = e.mult_exp.eval();
                        lhs - rhs
                    },
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerAdditiveExpression {
    exp: Box<AdditiveExpression>,
    op: AdditiveOperator,
    mult_exp: Box<MultiplicativeExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiplicativeExpression {
    Casting(Box<CastingExpression>),
    Mult(Box<InnerMultiplicativeExpression>)
}
impl AstNode<usize> for MultiplicativeExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Casting(e) => {
                e.eval()
            },
            Self::Mult(e) => {
                match &e.op {
                    MultiplicativeOperator::Mul => {
                        let lhs = e.exp.eval();
                        let rhs = e.casting_exp.eval();
                        lhs * rhs
                    },
                    MultiplicativeOperator::Div => {
                        let lhs = e.exp.eval();
                        let rhs = e.casting_exp.eval();
                        lhs / rhs
                    },
                    MultiplicativeOperator::Mod => {
                        let lhs = e.exp.eval();
                        let rhs = e.casting_exp.eval();
                        lhs % rhs
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerMultiplicativeExpression {
    exp: Box<MultiplicativeExpression>,
    op: MultiplicativeOperator,
    casting_exp: Box<CastingExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiplicativeOperator {
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CastingExpression {
    Unary(Box<UnaryExpression>),
    Casting(Box<InnerCastingExpression>),
}
impl AstNode<usize> for CastingExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Casting(e) => {
                e.exp.eval()
            },
            Self::Unary(e) => {
                e.eval()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerCastingExpression {
    exp: Box<CastingExpression>,
    op: CastingOperator,
    type_annotation: Box<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CastingOperator {
    Casting,            // as
    FailableCasting,    // as?
    ForceCasting,       // as!
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryExpression {
    Primary(Box<PrimaryExpression>),
    Unary(Box<InnerUnaryExpression>),
}
impl AstNode<usize> for UnaryExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Primary(e) => {
                e.eval()
            },
            Self::Unary(e) => {
                e.exp.eval()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerUnaryExpression {
    pub unary_operator: Vec<UnaryOperator>,
    pub exp: Box<UnaryExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Negate,
    Move,
}


#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryExpression {
    Value(usize),
}
impl AstNode<usize> for PrimaryExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Value(n) => *n
        }
    }
}