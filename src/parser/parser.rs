use crate::{lexer::token::{Token, TokenKind, Keyword}, ast::ast::{Node, NodeKind, BinaryOperation, UnaryOperation, FullType, ArgumentExp, Expression}};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    EOF,
    ParseError(String)
}

pub struct BacktrackingParser {
    pub cursor: usize,
    pub tokens: Vec<Token>
}

impl BacktrackingParser {
    pub fn new(tokens: &[Token]) -> Self {
        BacktrackingParser { cursor:0, tokens: Vec::from(tokens) }
    }
}

impl BacktrackingParser {
    fn back(&mut self) -> Result<Token, Error> {
        if self.cursor == 0 {
            return Err(Error::ParseError("can't back from begining".to_owned()))
        }
        self.cursor -= 1;
        Ok(self.tokens[self.cursor].clone())   
    }

    fn read(&mut self) -> Result<Token, Error> {
        if self.cursor >= self.tokens.len() {
            return Err(Error::EOF)
        }
        let current = self.cursor;
        self.cursor += 1;
        Ok(self.tokens[current].clone())
    }

    fn peek(&self) -> Option<Token> {
        if self.cursor >= self.tokens.len() {
            return None
        }
        Some(self.tokens[self.cursor].clone())
    }

    #[allow(dead_code)]
    fn peekn(&self, n: usize) -> Option<Token> {
        let cursor = self.cursor + n;
        if cursor >= self.tokens.len() {
            return None
        }
        Some(self.tokens[cursor].clone())
    }

    fn expect_and_read(&mut self, kind: TokenKind) -> Result<Token, Error> {
        let token = self.read()?;
        if token.kind == kind {
            Ok(token.clone())
        } else {
            let err = format!("expected {:?} but got {:?}", kind, token.kind);
            Err(Error::ParseError(err))
        }
    }
}

impl BacktrackingParser {
    pub fn expression(&mut self) -> Result<Node, Error> {
        let node = self.logical_term()?;
        Ok(Node::new(NodeKind::Expression(Expression::new(node))))
    }

    fn logical_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.eq_term()?;

        loop {
            if let Some(op) = self.logical_op() {
                self.read()?;
                let logi_op = match op.kind {
                    TokenKind::LogicalConjunction => BinaryOperation::AndAnd,
                    TokenKind::LogicalDisjunction => BinaryOperation::OrOr,
                    _ => return Err(Error::ParseError(format!("expected && || but got {:?}", op.kind)))
                };
                let rhs = self.eq_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), logi_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn eq_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.comp_term()?;

        loop {
            if let Some(op) = self.eq_op() {
                self.read()?;
                let eq_op = match op.kind {
                    TokenKind::EqualTo => BinaryOperation::Eq,
                    TokenKind::NotEqual => BinaryOperation::Neq,
                    _ => return Err(Error::ParseError(format!("expected = != but got {:?}", op.kind)))
                };
                let rhs = self.comp_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), eq_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn comp_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.nil_co_term()?;

        loop {
            if let Some(op) = self.comp_op() {
                self.read()?;
                let comp_op = match op.kind {
                    TokenKind::LessThan => BinaryOperation::Lt,
                    TokenKind::LessThanOrEqual => BinaryOperation::Lte,
                    TokenKind::GreaterThan => BinaryOperation::Gt,
                    TokenKind::GreaterThanOrEqual => BinaryOperation::Gte,
                    _ => return Err(Error::ParseError(format!("expected < <= > >= but got {:?}", op.kind)))
                };
                let rhs = self.nil_co_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), comp_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn nil_co_term(&mut self) -> Result<Node, Error> {
        let lhs = self.or_term()?;

        let mut rhs_vec = vec![];
        loop {
            if let Some(op) = self.nilco_op() {
                self.read()?;
                if op.kind != TokenKind::QuestionDouble {
                    return Err(Error::ParseError(format!("expected ?? but got {:?}", op.kind)))
                }
                let rhs = self.or_term()?;
                rhs_vec.push(rhs); // to handle right-associative nature of the operator
            } else {
                break;
            }
        }

        // merge rhs from right to left if any.
        if rhs_vec.len() > 0 {
            let mut rhs = rhs_vec.pop().unwrap();
            loop {
                if let Some(next) = rhs_vec.pop() {
                    rhs = Node::new(NodeKind::BinaryOperation(Box::new(next), Box::new(rhs), BinaryOperation::NilCo))
                } else {
                    break;
                }
            }
            return Ok(Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), BinaryOperation::NilCo)))
        }

        Ok(lhs)
    }

    fn or_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.xor_term()?;

        loop {
            if let Some(op) = self.or_op() {
                self.read()?;
                let or_op = match op.kind {
                    TokenKind::BitwiseOr => BinaryOperation::Or,
                    _ => return Err(Error::ParseError(format!("expected | but got {:?}", op.kind)))
                };
                let rhs = self.xor_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), or_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn xor_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.and_term()?;

        loop {
            if let Some(op) = self.xor_op() {
                self.read()?;
                let xor_op = match op.kind {
                    TokenKind::Xor => BinaryOperation::Xor,
                    _ => return Err(Error::ParseError(format!("expected ^ but got {:?}", op.kind)))
                };
                let rhs = self.and_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), xor_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn and_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.shift_term()?;

        loop {
            if let Some(op) = self.and_op() {
                self.read()?;
                let and_op = match op.kind {
                    TokenKind::BitwiseAnd => BinaryOperation::And,
                    _ => return Err(Error::ParseError(format!("expected & but got {:?}", op.kind)))
                };
                let rhs = self.shift_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), and_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn shift_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.additive_term()?;

        loop {
            if let Some(op) = self.shift_op() {
                self.read()?;
                let shift_op = match op.kind {
                    TokenKind::BitwiseShiftLeft => BinaryOperation::Shil,
                    TokenKind::BitwiseShiftRight => BinaryOperation::Shir,
                    _ => return Err(Error::ParseError(format!("expected << >> but got {:?}", op.kind)))
                };
                let rhs = self.additive_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), shift_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn additive_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.multiplicative_term()?;
        
        loop {
            if let Some(op) = self.additive_op() {
                self.read()?;
                let add_op = match op.kind {
                    TokenKind::Plus => BinaryOperation::Add,
                    TokenKind::Minus => BinaryOperation::Sub,
                    _ => return Err(Error::ParseError(format!("expected + - but got {:?}", op.kind)))
                };
                let rhs = self.multiplicative_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), add_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn multiplicative_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.as_term()?;
        
        loop {
            if let Some(op) = self.multiplicative_op() {
                self.read()?;
                let mul_op = match op.kind {
                    TokenKind::Asterisk => BinaryOperation::Mul,
                    TokenKind::Slash => BinaryOperation::Div,
                    TokenKind::Percent => BinaryOperation::Mod,
                    _ => return Err(Error::ParseError(format!("expected * / % but got {:?}", op.kind)))
                };
                let rhs = self.as_term()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), mul_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn as_term(&mut self) -> Result<Node, Error> {
        let mut lhs = self.unary_term()?;

        loop {
            if let Some(op) = self.as_op() {
                self.read()?;
                let as_op = match op.kind {
                    TokenKind::As => BinaryOperation::As,
                    TokenKind::AsEx => BinaryOperation::AsExclamation,
                    TokenKind::AsQu => BinaryOperation::AsQuestion,
                    _ => return Err(Error::ParseError(format!("expected as as! as? but got {:?}", op.kind)))
                };
                let rhs = self.type_annotation()?;
                lhs = Node::new(NodeKind::BinaryOperation(Box::new(lhs), Box::new(rhs), as_op))
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn unary_term(&mut self) -> Result<Node, Error> {
        let mut unary_ops = vec![];
        loop {
            if let Some(op) = self.unary_op() {
                self.read()?;
                let unary_op = match op.kind {
                    TokenKind::Minus => UnaryOperation::Minus,
                    TokenKind::Exclamation => UnaryOperation::Negate,
                    TokenKind::Move => UnaryOperation::Move,
                    _ => return Err(Error::ParseError(format!("expected - ! <- but got {:?}", op.kind)))
                };
                unary_ops.push(unary_op);
            } else {
                break;
            }
        }
        let operand = self.primary_term()?;
        Ok(Node::new(NodeKind::UnaryOperation(Box::new(operand), unary_ops)))
    }

    fn primary_term(&mut self) -> Result<Node, Error> {
        let token = self.read()?;
        match token.kind {
            TokenKind::Keyword(Keyword::Create) => self.create_term(),
            TokenKind::Keyword(Keyword::Destroy) => self.destroy_term(),
            TokenKind::BitwiseAnd => self.reference_term(),
            _ => {
                self.back()?;
                self.factor()
            },
        }
    }

    fn create_term(&mut self) -> Result<Node, Error> {
        Err(Error::ParseError("TODO: Create Exp not implemented".to_string()))
    }

    fn destroy_term(&mut self) -> Result<Node, Error> {
        let node = self.expression()?;
        Ok(Node::new(NodeKind::Destroy(Box::new(node))))
    }

    fn reference_term(&mut self) -> Result<Node, Error> {
        Err(Error::ParseError("TODO: Reference Exp not implemented".to_string()))
    }

    fn factor(&mut self) -> Result<Node, Error> {
        let token = self.read()?;
        if token.kind == TokenKind::ParenOpen {
            let node = self.expression()?;
            self.expect_and_read(TokenKind::ParenClose)?;
            return Ok(node)
        } else {
            self.back()?;
            let node = self.terminal()?;
            return Ok(node)
        }
    }

    fn terminal(&mut self) -> Result<Node, Error> {
        let token = self.read()?;
        match token.kind {
            TokenKind::String(str) => Ok(Node::new(NodeKind::TerminalString(str))),
            TokenKind::Identifier(id) => Ok(Node::new(NodeKind::TerminalIdentifier(id))),
            TokenKind::Number(num) => Ok(Node::new(NodeKind::TerminalNumber(num))),
            _ => Err(Error::ParseError("not terminal".to_owned()))
        }
    }
}

impl BacktrackingParser {
    fn logical_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::LogicalConjunction | TokenKind::LogicalDisjunction => Some(token),
                _ => None,
            }
        }
        None
    }
    fn eq_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::EqualTo | TokenKind::NotEqual => Some(token),
                _ => None,
            }
        }
        None
    }
    fn comp_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::LessThan | TokenKind::LessThanOrEqual | TokenKind::GreaterThan | TokenKind::GreaterThanOrEqual => Some(token),
                _ => None,
            }
        }
        None
    }
    fn nilco_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::QuestionDouble => Some(token),
                _ => None,
            }
        }
        None
    }
    fn or_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::BitwiseOr => Some(token),
                _ => None,
            }
        }
        None
    }
    fn xor_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::Xor => Some(token),
                _ => None,
            }
        }
        None
    }
    fn and_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::BitwiseAnd => Some(token),
                _ => None,
            }
        }
        None
    }
    fn shift_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::BitwiseShiftLeft | TokenKind::BitwiseShiftRight => Some(token),
                _ => None,
            }
        }
        None
    }
    fn additive_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::Plus | TokenKind::Minus => Some(token),
                _ => None,
            }
        }
        None
    }
    fn multiplicative_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => Some(token),
                _ => None,
            }
        }
        None
    }
    fn as_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::As | TokenKind::AsEx | TokenKind::AsQu => Some(token),
                _ => None,
            }
        }
        None
    }
    fn unary_op(&mut self) -> Option<Token> {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::Minus | TokenKind::Exclamation | TokenKind::Move => Some(token),
                _ => None,
            }
        }
        None
    }
}

impl BacktrackingParser {
    pub(crate) fn type_annotation(&mut self) -> Result<Node, Error> {
        let mut token = self.read()?;
        let is_resource = if token.kind == TokenKind::At {
                token = self.read()?;
                true
            } else {
                false
            };
        
        let is_auth = if token.kind == TokenKind::Keyword(Keyword::Auth) {
                token = self.read()?;
                true
            } else {
                false
            };
        
        let is_ref = if token.kind == TokenKind::BitwiseAnd {
                token = self.read()?;
                true
            } else {
                false
            };
        
        match token.kind {
            TokenKind::Identifier(id) => {
                match (is_resource, is_auth, is_ref) {
                    (true, true, true) => Ok(Node::new(NodeKind::ResourceTypeAnnotation(FullType::AuthReference(id)))),
                    (true, true, false) => Err(Error::ParseError("expect & after auth".to_string())),
                    (true, false, true) => Ok(Node::new(NodeKind::ResourceTypeAnnotation(FullType::Reference(id)))),
                    (true, false, false) => Ok(Node::new(NodeKind::ResourceTypeAnnotation(FullType::Inner(id)))),

                    (false, true, true) => Ok(Node::new(NodeKind::TypeAnnotation(FullType::AuthReference(id)))),
                    (false, true, false) => Err(Error::ParseError("expect & after auth".to_string())),
                    (false, false, true) => Ok(Node::new(NodeKind::TypeAnnotation(FullType::Reference(id)))),
                    (false, false, false) => Ok(Node::new(NodeKind::TypeAnnotation(FullType::Inner(id)))),
                }
            },
            _ => Err(Error::ParseError(format!("expect type got {:?}", token.kind)))
        }

    }

    // pub(crate) fn invocation(&mut self) -> Result<Node, Error> {
    //     let tok = self.read()?;
    //     if tok.kind == TokenKind::AngleOpen {

    //     }
    // }

    pub(crate) fn argument(&mut self) -> Result<Node, Error> {
        let tok = self.read()?;
        match tok.kind {
            TokenKind::Identifier(id) => {
                self.expect_and_read(TokenKind::Colon)?;
                let exp = self.expression()?;
                Ok(Node::new(NodeKind::Argument(ArgumentExp::new(Some(id), exp))))
            },
            _ => {
                self.back()?;
                let exp = self.expression()?;
                Ok(Node::new(NodeKind::Argument(ArgumentExp::new(None, exp))))
            }
        }
    }
}