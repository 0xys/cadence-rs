pub mod parser;

#[cfg(test)]
mod tests {   
    use crate::parser::parser::BacktrackingParser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test() {
        let code  = "1+2*3";
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        let mut parser = BacktrackingParser::new(&tokens);
        let ast = parser.expression();

        let res = match ast {
            Ok(node) => format!("{}", node),
            Err(e) => format!("{:?}", e)
        };

        assert_eq!(res, "Add{num{1}, Mul{num{2}, num{3}}}")
    }

    #[test]
    fn test_with_paren() {
        let code  = "(1 + (2+3)*4 )  +  5/6 * (7+8) % 9";
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        let mut parser = BacktrackingParser::new(&tokens);
        let ast = parser.expression();

        let res = match ast {
            Ok(node) => format!("{}", node),
            Err(e) => format!("{:?}", e)
        };

        assert_eq!(res, "Add{Add{num{1}, Mul{Add{num{2}, num{3}}, num{4}}}, Mod{Mul{Div{num{5}, num{6}}, Add{num{7}, num{8}}}, num{9}}}");
    }

    #[test]
    fn test_with_paren_2() {
        let code  = "(1 + (2+3)*4 )  +  5/6 * ((7+8) % 9)";
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        let mut parser = BacktrackingParser::new(&tokens);
        let ast = parser.expression();

        let res = match ast {
            Ok(node) => format!("{}", node),
            Err(e) => format!("{:?}", e)
        };
        

        assert_eq!(res, "Add{Add{num{1}, Mul{Add{num{2}, num{3}}, num{4}}}, Mul{Div{num{5}, num{6}}, Mod{Add{num{7}, num{8}}, num{9}}}}");
    }
}