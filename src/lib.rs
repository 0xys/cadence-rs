pub mod lexer;

#[cfg(test)]
mod tests {    
    use crate::lexer::Lexer;

    #[test]
    fn it_works() {
        let code = "abc";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.peek(), Some(b'a'));
        assert_eq!(lexer.read(), Some(b'a'));

        assert_eq!(lexer.peek(), Some(b'b'));
        assert_eq!(lexer.read(), Some(b'b'));

        assert_eq!(lexer.peek(), Some(b'c'));
        assert_eq!(lexer.read(), Some(b'c'));

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.read(), None);
    }
}
