pub mod parser;

#[cfg(test)]
mod tests {   
    use crate::ast::ast::Node;
    use crate::parser::parser::BacktrackingParser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test() {
        let code  = "1+2*3";
        let ast = gen_ast(code);

        assert_eq!(ast.to_string(), "Add{num(1), Mul{num(2), num(3)}}")
    }

    #[test]
    fn test_with_paren() {
        let code  = "(1 + (2+3)*4 )  +  5/6 * (7+8) % 9";
        let ast = gen_ast(code);

        assert_eq!(ast.to_string(), "Add{Add{num(1), Mul{Add{num(2), num(3)}, num(4)}}, Mod{Mul{Div{num(5), num(6)}, Add{num(7), num(8)}}, num(9)}}");
    }

    #[test]
    fn test_with_paren_2() {
        let code  = "(1 + (2+3)*4 )  +  5/6 * ((7+8) % 9)";
        let ast = gen_ast(code);

        assert_eq!(ast.to_string(), "Add{Add{num(1), Mul{Add{num(2), num(3)}, num(4)}}, Mul{Div{num(5), num(6)}, Mod{Add{num(7), num(8)}, num(9)}}}");
    }

    #[test]
    fn test_shift() {
        let code  = "1 << 2 >> 3 + 4";
        let ast = gen_ast(code);

        assert_eq!(ast.to_string(), "Shir{Shil{num(1), num(2)}, Add{num(3), num(4)}}");
    }

    #[test]
    fn test_nil_coerce() {
        let code  = "1 ?? 2";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "NilCo{num(1), num(2)}");

        let code  = "1 ?? 2 ?? 3";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "NilCo{num(1), NilCo{num(2), num(3)}}");
        
        let code  = "1*2 ?? 3+4 ?? 5 << 6 ?? 7&8 ?? 9^10 ?? 11|12";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "NilCo{Mul{num(1), num(2)}, NilCo{Add{num(3), num(4)}, NilCo{Shil{num(5), num(6)}, NilCo{And{num(7), num(8)}, NilCo{Xor{num(9), num(10)}, Or{num(11), num(12)}}}}}}");
    }

    #[test]
    fn test_as() {
        let code  = "1 ?? 2 + 3 as 4 * 5";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "NilCo{num(1), Add{num(2), Mul{As{num(3), num(4)}, num(5)}}}");
    }

    fn gen_ast(code: &str) -> Node {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        let mut parser = BacktrackingParser::new(&tokens);
        let ast = parser.expression();
        ast.unwrap().clone()
    }
}