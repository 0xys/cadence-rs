use crate::lexer::token::Token;

pub trait Parser {
    fn read(&mut self) -> Option<Token>;
    fn peek(&self) -> Option<Token>;
    fn peekn(&self, n: usize) -> Option<Token>;
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

impl Parser for BacktrackingParser {
    fn read(&mut self) -> Option<Token> {
        todo!()
    }

    fn peek(&self) -> Option<Token> {
        todo!()
    }

    fn peekn(&self, n: usize) -> Option<Token> {
        todo!()
    }
}

pub fn parse_expression(p: &mut dyn Parser) {

}