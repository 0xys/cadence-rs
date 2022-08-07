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
        let code  = "a as A";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "As{id(a), Type{A}}");

        let code  = "a as &A";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "As{id(a), Type{&A}}");

        let code  = "a as auth &A";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "As{id(a), Type{auth &A}}");

        let code  = "1 ?? 2 + 3 as A";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "NilCo{num(1), Add{num(2), As{num(3), Type{A}}}}");
    }

    #[test]
    fn test_unary() {
        let code  = "-1";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "[Minus]{num(1)}");

        let code  = "1 + -1";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "Add{num(1), [Minus]{num(1)}}");

        let code  = "-!<--1";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "[Minus, Negate, Move, Minus]{num(1)}");

        let code  = "-1*-2";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "Mul{[Minus]{num(1)}, [Minus]{num(2)}}");
    }

    #[test]
    fn test_destroy() {
        let code  = "destroy a";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "Destroy{id(a)}");

        let code  = "-1*-2+destroy a";
        let ast = gen_ast(code);
        assert_eq!(ast.to_string(), "Add{Mul{[Minus]{num(1)}, [Minus]{num(2)}}, Destroy{id(a)}}");
    }

    #[test]
    fn test_parse_type_annotation() {
        fn gen_type(code: &str) -> Option<Node> {
            let mut lexer = Lexer::new(code);
            let tokens = lexer.tokenize_all();

            let mut parser = BacktrackingParser::new(&tokens);
            let ast = parser.type_annotation();
            if ast.is_err() {
                None
            } else {
                Some(ast.unwrap().clone())
            }
        }

        let code = "a";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "Type{a}");

        let code = "&a";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "Type{&a}");

        let code = "auth &ab";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "Type{auth &ab}");

        let code = "@a";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "@Type{a}");

        let code = "@&a";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "@Type{&a}");

        let code = "@auth&a";
        let ast = gen_type(code);
        assert_eq!(ast.unwrap().to_string(), "@Type{auth &a}");
    }

    #[test]
    fn test_parse_argument() {
        fn gen_argument(code: &str) -> Option<Node> {
            let mut lexer = Lexer::new(code);
            let tokens = lexer.tokenize_all();

            let mut parser = BacktrackingParser::new(&tokens);
            let ast = parser.argument();
            if ast.is_err() {
                None
            } else {
                Some(ast.unwrap().clone())
            }
        }

        let code = "1+2*3";
        let ast = gen_argument(code);
        assert_eq!(ast.unwrap().to_string(), "Arg{Add{num(1), Mul{num(2), num(3)}}}");

        let code = "a: 1+2*3";
        let ast = gen_argument(code);
        assert_eq!(ast.unwrap().to_string(), "Arg{a: Add{num(1), Mul{num(2), num(3)}}}");
    }

    fn gen_ast(code: &str) -> Node {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        let mut parser = BacktrackingParser::new(&tokens);
        let ast = parser.expression();
        ast.unwrap().clone()
    }
}