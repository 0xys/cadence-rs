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

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalExpression {
    nodes: Vec<OrExpression>
}
impl ConditionalExpression {
    pub fn new(nodes: Vec<OrExpression>) -> Self {
        Self {
            nodes,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrExpression {
    nodes: Vec<AndExpression>
}
impl OrExpression {
    pub fn new(nodes: Vec<AndExpression>) -> Self {
        Self {
            nodes,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AndExpression {
    nodes: Vec<EqualExpression>
}
impl AndExpression {
    pub fn new(nodes: Vec<EqualExpression>) -> Self {
        Self {
            nodes,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EqualExpression {
    nodes: Vec<>
}
impl EqualExpression {
    pub fn new(nodes: Vec<>) -> Self {
        Self {
            nodes,
        }
    }
}


