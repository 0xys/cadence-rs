pub mod lexer;

#[cfg(test)]
mod tests {    
    use crate::lexer::lexer::Lexer;
    use crate::lexer::token::TokenKind;

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

    #[test]
    fn it_works_2() {
        let code = "()[]{}<>,.+-*/\\%=?!@&|^\"';:";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.peek(), Some(b'('));
        assert_eq!(lexer.tokenize().kind, TokenKind::ParenOpen);

        assert_eq!(lexer.peek(), Some(b')'));
        assert_eq!(lexer.tokenize().kind, TokenKind::ParenClose);

        assert_eq!(lexer.peek(), Some(b'['));
        assert_eq!(lexer.tokenize().kind, TokenKind::BracketOpen);

        assert_eq!(lexer.peek(), Some(b']'));
        assert_eq!(lexer.tokenize().kind, TokenKind::BracketClose);

        assert_eq!(lexer.peek(), Some(b'{'));
        assert_eq!(lexer.tokenize().kind, TokenKind::BraceOpen);

        assert_eq!(lexer.peek(), Some(b'}'));
        assert_eq!(lexer.tokenize().kind, TokenKind::BraceClose);

        assert_eq!(lexer.peek(), Some(b'<'));
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleOpen);

        assert_eq!(lexer.peek(), Some(b'>'));
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleClose);

        assert_eq!(lexer.peek(), Some(b','));
        assert_eq!(lexer.tokenize().kind, TokenKind::Comma);

        assert_eq!(lexer.peek(), Some(b'.'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Period);

        assert_eq!(lexer.peek(), Some(b'+'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Plus);

        assert_eq!(lexer.peek(), Some(b'-'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Minus);

        assert_eq!(lexer.peek(), Some(b'*'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Asterisk);

        assert_eq!(lexer.peek(), Some(b'/'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Slash);

        assert_eq!(lexer.peek(), Some(b'\\'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Backslash);

        assert_eq!(lexer.peek(), Some(b'%'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Percent);

        assert_eq!(lexer.peek(), Some(b'='));
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);

        assert_eq!(lexer.peek(), Some(b'?'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Question);

        assert_eq!(lexer.peek(), Some(b'!'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Exclamation);

        assert_eq!(lexer.peek(), Some(b'@'));
        assert_eq!(lexer.tokenize().kind, TokenKind::At);

        assert_eq!(lexer.peek(), Some(b'&'));
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseAnd);

        assert_eq!(lexer.peek(), Some(b'|'));
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseOr);

        assert_eq!(lexer.peek(), Some(b'^'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Xor);

        assert_eq!(lexer.read(), Some(b'"'));
        // assert_eq!(lexer.tokenize().kind, TokenKind::DoubleQuote));

        assert_eq!(lexer.peek(), Some(b'\''));
        assert_eq!(lexer.tokenize().kind, TokenKind::SingleQuote);

        assert_eq!(lexer.peek(), Some(b';'));
        assert_eq!(lexer.tokenize().kind, TokenKind::SemiColon);

        assert_eq!(lexer.peek(), Some(b':'));
        assert_eq!(lexer.tokenize().kind, TokenKind::Colon);

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.tokenize().kind, TokenKind::EOF);
    }

    #[test]
    fn test_skip_line_comments() {
        let code = "+//abv/\n/{///)\n/@//";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::Plus);
        assert_eq!(lexer.tokenize().kind, TokenKind::Slash);
        assert_eq!(lexer.tokenize().kind, TokenKind::BraceOpen);
        assert_eq!(lexer.tokenize().kind, TokenKind::Slash);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::EOF);
    }

    #[test]
    fn test_tokenize_neq() {
        let code = "!+!= ! =&";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::Exclamation);
        assert_eq!(lexer.tokenize().kind, TokenKind::Plus);
        assert_eq!(lexer.tokenize().kind, TokenKind::NotEqual);
        assert_eq!(lexer.tokenize().kind, TokenKind::Exclamation);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseAnd);
    }

    #[test]
    fn test_tokenize_lte_gte() {
        let code = "< <=@> >=  <  >";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::AngleOpen);
        assert_eq!(lexer.tokenize().kind, TokenKind::LessThanOrEqual);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleClose);
        assert_eq!(lexer.tokenize().kind, TokenKind::GreaterThanOrEqual);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleOpen);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleClose);
    }

    #[test]
    fn test_tokenize_bitshift() {
        let code = "<<=@>>=  <  > <<< <=";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseShiftLeft);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseShiftRight);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleOpen);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleClose);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseShiftLeft);
        assert_eq!(lexer.tokenize().kind, TokenKind::AngleOpen);
        assert_eq!(lexer.tokenize().kind, TokenKind::LessThanOrEqual);
    }

    #[test]
    fn test_tokenize_equal() {
        let code = "= == @ = @ == ===";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::EqualTo);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::EqualTo);
        assert_eq!(lexer.tokenize().kind, TokenKind::EqualTo);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
    }

    #[test]
    fn test_tokenize_and_or() {
        let code = "@ & @ @ && @ &= | | || @ |=|";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseAnd);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::LogicalConjunction);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseAnd);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseOr);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseOr);
        assert_eq!(lexer.tokenize().kind, TokenKind::LogicalDisjunction);
        assert_eq!(lexer.tokenize().kind, TokenKind::At);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseOr);
        assert_eq!(lexer.tokenize().kind, TokenKind::Assign);
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseOr);
    }

    #[test]
    fn test_tokenize_identifier() {
        let code = ". abc AbC 0 123 a1 a_3 3.1 1.a a&1 abc. _d3 123. 2";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::None); // period with whitespaces are None
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("abc".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("AbC".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("123".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("a1".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("a_3".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("3.1".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("1.".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("a".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("a".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::BitwiseAnd);
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("1".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("abc".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::None);
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("_d3".to_string()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("123".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::None); // period with whitespaces are None
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("2".to_string()));
    }

    #[test]
    fn test_tokenize_bits_hex() {
        let code = "0xa 0b10 0bb 0xx 0xAd 0.b0b 00b1 00xa";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0xa".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0b10".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0b".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("b".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0x".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("x".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0xAd".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("0.".to_owned()));
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("b0b".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("00".to_owned()));
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("b1".to_owned()));

        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("00".to_owned()));
        assert_eq!(lexer.tokenize().kind, TokenKind::Identifier("xa".to_owned()));
    }

    #[test]
    fn test_tokenize_double_quote() {
        // `` `h` `1  anvcd*'_d` `"` `\n` `"hello\nworld"` `<EOF>
        let code = "\"\" \"h\" \"1  anvcd*'_d\" \"\\\"\" \"\\n\" \"\\\\\" \"\\\"hello\\nworld\\\"\" \"";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize().kind, TokenKind::String("".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("h".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("1  anvcd*'_d".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("\"".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("\n".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("\\".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::String("\"hello\nworld\"".to_string()));
        assert_eq!(lexer.tokenize().kind, TokenKind::DoubleQuote);
    }
}
