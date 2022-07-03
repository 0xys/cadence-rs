pub mod lexer;

#[cfg(test)]
mod tests {    
    use crate::lexer::tokenizer::Lexer;
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
        assert_eq!(lexer.tokenize(), Some(TokenKind::ParenOpen));

        assert_eq!(lexer.peek(), Some(b')'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::ParenClose));

        assert_eq!(lexer.peek(), Some(b'['));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BracketOpen));

        assert_eq!(lexer.peek(), Some(b']'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BracketClose));

        assert_eq!(lexer.peek(), Some(b'{'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BraceOpen));

        assert_eq!(lexer.peek(), Some(b'}'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BraceClose));

        assert_eq!(lexer.peek(), Some(b'<'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleOpen));

        assert_eq!(lexer.peek(), Some(b'>'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleClose));

        assert_eq!(lexer.peek(), Some(b','));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Comma));

        assert_eq!(lexer.peek(), Some(b'.'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b'+'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Plus));

        assert_eq!(lexer.peek(), Some(b'-'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Minus));

        assert_eq!(lexer.peek(), Some(b'*'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Asterisk));

        assert_eq!(lexer.peek(), Some(b'/'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Slash));

        assert_eq!(lexer.peek(), Some(b'\\'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Backslash));

        assert_eq!(lexer.peek(), Some(b'%'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Percent));

        assert_eq!(lexer.peek(), Some(b'='));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));

        assert_eq!(lexer.peek(), Some(b'?'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Question));

        assert_eq!(lexer.peek(), Some(b'!'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Exclamation));

        assert_eq!(lexer.peek(), Some(b'@'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));

        assert_eq!(lexer.peek(), Some(b'&'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseAnd));

        assert_eq!(lexer.peek(), Some(b'|'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseOr));

        assert_eq!(lexer.peek(), Some(b'^'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Xor));

        assert_eq!(lexer.read(), Some(b'"'));
        // assert_eq!(lexer.tokenize(), Some(TokenKind::DoubleQuote));

        assert_eq!(lexer.peek(), Some(b'\''));
        assert_eq!(lexer.tokenize(), Some(TokenKind::SingleQuote));

        assert_eq!(lexer.peek(), Some(b';'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::SemiColon));

        assert_eq!(lexer.peek(), Some(b':'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Colon));

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.tokenize(), None);
    }

    #[test]
    fn it_works_3() {
        let code = ".. .  .   .    .     .      .    ";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.peek(), Some(b'.'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b'.'));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));

        assert_eq!(lexer.peek(), Some(b' '));
        assert_eq!(lexer.tokenize(), None);
    }

    #[test]
    fn test_skip_line_comments() {
        let code = "+//abv/\n/{///)\n/";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::Plus));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Slash));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BraceOpen));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Slash));
    }

    #[test]
    fn test_tokenize_neq() {
        let code = "!+!= ! =&";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::Exclamation));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Plus));
        assert_eq!(lexer.tokenize(), Some(TokenKind::NotEqual));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Exclamation));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseAnd));
    }

    #[test]
    fn test_tokenize_lte_gte() {
        let code = "< <=@> >=  <  >";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleOpen));
        assert_eq!(lexer.tokenize(), Some(TokenKind::LessThanOrEqual));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleClose));
        assert_eq!(lexer.tokenize(), Some(TokenKind::GreaterThanOrEqual));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleOpen));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleClose));
    }

    #[test]
    fn test_tokenize_bitshift() {
        let code = "<<=@>>=  <  > <<< <=";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseShiftLeft));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseShiftRight));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleOpen));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleClose));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseShiftLeft));
        assert_eq!(lexer.tokenize(), Some(TokenKind::AngleOpen));
        assert_eq!(lexer.tokenize(), Some(TokenKind::LessThanOrEqual));
    }

    #[test]
    fn test_tokenize_equal() {
        let code = "= == @ = @ == ===";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::EqualTo));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::EqualTo));
        assert_eq!(lexer.tokenize(), Some(TokenKind::EqualTo));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
    }

    #[test]
    fn test_tokenize_and_or() {
        let code = "@ & @ @ && @ &= | | || @ |=|";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseAnd));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::LogicalConjunction));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseAnd));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseOr));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseOr));
        assert_eq!(lexer.tokenize(), Some(TokenKind::LogicalDisjunction));
        assert_eq!(lexer.tokenize(), Some(TokenKind::At));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseOr));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Assign));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseOr));
    }

    #[test]
    fn test_tokenize_identifier() {
        let code = ". abc AbC 0 123 a1 a_3 3.1 1.a a&1 abc. _d3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("abc".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("AbC".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("123".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("a1".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("a_3".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("3.1".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("1.".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("a".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("a".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::BitwiseAnd));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("1".to_string())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("abc".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Period));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("_d3".to_string())));
    }

    #[test]
    fn test_tokenize_bits_hex() {
        let code = "0xa 0b10 0bb 0xx 0xAd 0.b0b 00b1 00xa";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0xa".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0b10".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0b".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("b".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0x".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("x".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0xAd".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("0.".to_owned())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("b0b".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("00".to_owned())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("b1".to_owned())));

        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("00".to_owned())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::Identifier("xa".to_owned())));
    }

    #[test]
    fn test_tokenize_double_quote() {
        let code = "\"\" \"h\" \"1  anvcd*'_d\" \"\\\"  \"";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.tokenize(), Some(TokenKind::String("".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::String("h".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::String("1  anvcd*'_d".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::String("\\".to_string())));
        assert_eq!(lexer.tokenize(), Some(TokenKind::DoubleQuote));
    }
}
