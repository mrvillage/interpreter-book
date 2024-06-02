#[cfg(test)]
mod tests {
    use crate::*;
    use TokenType::*;

    #[test]
    fn test_let_statements() {
        let input = String::from(
            r#"let x = 5;
let y = 10;
let foobar = 838383;"#,
        );
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        assert!(program.is_ok());
        let program = program.unwrap();
        assert_eq!(program.statements().len(), 3);
        let idents = ["x", "y", "foobar"];
        for (stmt, ident) in program.statements().iter().zip(idents.iter()) {
            assert_eq!(stmt.token_literal(), "let");
            assert_eq!(stmt.token_type(), Let);
            let let_stmt = stmt.as_any().downcast_ref::<LetStatement>().unwrap();
            assert_eq!(let_stmt.name().token_literal(), *ident);
            assert_eq!(let_stmt.name().token().token_type(), Ident);
            assert_eq!(let_stmt.name().value(), *ident);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = String::from(
            r#"return 5;
return 10;
return 993322;
"#,
        );

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        assert!(program.is_ok());
        let program = program.unwrap();
        assert_eq!(program.statements().len(), 3);
        for stmt in program.statements() {
            assert_eq!(stmt.token_literal(), "return");
            assert_eq!(stmt.token_type(), Return);
        }
    }

    #[test]
    fn test_identifier() {
        let input = String::from("foobar;");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements().len(), 1);
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        assert_eq!(stmt.token_literal(), "foobar");
        assert_eq!(stmt.token_type(), Ident);
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = String::from("5");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements().len(), 1);
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let literal = stmt
            .expression()
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .unwrap();
        assert_eq!(literal.value(), 5);
        assert_eq!(literal.token_literal(), "5");
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests = vec![("!5", "!", 5), ("-15", "-", 15)];
        for (input, operator, value) in prefix_tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements().len(), 1);
            let stmt = program
                .statements()
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let expr = stmt.expression();
            let prefix_expr = expr.as_any().downcast_ref::<PrefixExpression>().unwrap();
            assert_eq!(prefix_expr.operator(), operator);
            let right = prefix_expr.right();
            let int = right.as_any().downcast_ref::<IntegerLiteral>().unwrap();
            assert_eq!(int.value(), value);
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = vec![
            ("5 + 5", 5, "+", 5),
            ("5 - 5", 5, "-", 5),
            ("5 * 5", 5, "*", 5),
            ("5 / 5", 5, "/", 5),
            ("5 > 5", 5, ">", 5),
            ("5 < 5", 5, "<", 5),
            ("5 == 5", 5, "==", 5),
            ("5 != 5", 5, "!=", 5),
        ];

        for (input, expected_left, op, expected_right) in infix_tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements().len(), 1);
            let stmt = program
                .statements()
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let expr = stmt.expression();
            let infix_expr = expr.as_any().downcast_ref::<InfixExpression>().unwrap();
            let left = infix_expr.left();
            let int = left.as_any().downcast_ref::<IntegerLiteral>().unwrap();
            assert_eq!(int.value(), expected_left);
            assert_eq!(infix_expr.operator(), op);
            let right = infix_expr.right();
            let int = right.as_any().downcast_ref::<IntegerLiteral>().unwrap();
            assert_eq!(int.value(), expected_right);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let precedence_tests = vec![
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 < 4 == 3 < 4", "((5 < 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];

        for (input, output) in precedence_tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let actual = program.to_string().trim().to_string();
            assert_eq!(actual, output);
        }
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![("true", true), ("false", false)];

        for (input, expected) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements().len(), 1);
            let stmt = program
                .statements()
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let expr = stmt.expression();
            let bool_expr = expr.as_any().downcast_ref::<Boolean>().unwrap();
            assert_eq!(bool_expr.value(), expected);
        }
    }

    #[test]
    fn test_if_expression() {
        let input = String::from("if (x < y) { x }");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements().len(), 1);
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let if_expr = expr.as_any().downcast_ref::<IfExpression>().unwrap();
        let condition = if_expr.condition();
        let infix_expr = condition
            .as_any()
            .downcast_ref::<InfixExpression>()
            .unwrap();
        let left = infix_expr.left();
        let ident = left.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "x");
        assert_eq!(infix_expr.operator(), "<");
        let right = infix_expr.right();
        let ident = right.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "y");
        let consequence = if_expr.consequence();
        let block_stmt = consequence
            .as_any()
            .downcast_ref::<BlockStatement>()
            .unwrap();
        assert_eq!(block_stmt.statements().len(), 1);
        let stmt = block_stmt
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let ident = expr.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "x");
        assert!(if_expr.alternative().is_none());
    }

    #[test]
    fn test_if_else_expression() {
        let input = String::from("if (x < y) { x } else { y }");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements().len(), 1);
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let if_expr = expr.as_any().downcast_ref::<IfExpression>().unwrap();
        let condition = if_expr.condition();
        let infix_expr = condition
            .as_any()
            .downcast_ref::<InfixExpression>()
            .unwrap();
        let left = infix_expr.left();
        let ident = left.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "x");
        assert_eq!(infix_expr.operator(), "<");
        let right = infix_expr.right();
        let ident = right.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "y");
        let consequence = if_expr.consequence();
        let block_stmt = consequence
            .as_any()
            .downcast_ref::<BlockStatement>()
            .unwrap();
        assert_eq!(block_stmt.statements().len(), 1);
        let stmt = block_stmt
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let ident = expr.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "x");
        let alternative = if_expr.alternative().as_ref().unwrap();
        let block_stmt = alternative
            .as_any()
            .downcast_ref::<BlockStatement>()
            .unwrap();
        assert_eq!(block_stmt.statements().len(), 1);
        let stmt = block_stmt
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let ident = expr.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "y");
    }

    #[test]
    fn test_function_literal() {
        let input = String::from("fn(x, y) { x + y; }");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements().len(), 1);
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let func = expr.as_any().downcast_ref::<FunctionLiteral>().unwrap();
        assert_eq!(func.parameters().len(), 2);
        let params = ["x", "y"];
        for (param, expected) in func.parameters().iter().zip(params.iter()) {
            let ident = param.as_any().downcast_ref::<Identifier>().unwrap();
            assert_eq!(ident.value(), *expected);
        }
        let body = func.body();
        let block_stmt = body.as_any().downcast_ref::<BlockStatement>().unwrap();
        assert_eq!(block_stmt.statements().len(), 1);
        let stmt = block_stmt
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let infix_expr = expr.as_any().downcast_ref::<InfixExpression>().unwrap();
        let left = infix_expr.left();
        let ident = left.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "x");
        assert_eq!(infix_expr.operator(), "+");
        let right = infix_expr.right();
        let ident = right.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(ident.value(), "y");
    }

    #[test]
    fn test_function_parameter_parsing() {
        let input = vec![
            ("fn() {};", vec![]),
            ("fn(x) {};", vec!["x"]),
            ("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];
        for (input, expected_params) in input {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let stmt = program
                .statements()
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let expr = stmt.expression();
            let func = expr.as_any().downcast_ref::<FunctionLiteral>().unwrap();
            assert_eq!(func.parameters().len(), expected_params.len());
            for (ident, expected) in func.parameters().iter().zip(expected_params.iter()) {
                assert_eq!(ident.value(), *expected);
            }
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = String::from("add(1, 2 * 3, 4 + 5);");
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let stmt = program
            .statements()
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let expr = stmt.expression();
        let call_expr = expr.as_any().downcast_ref::<CallExpression>().unwrap();
        let ident = call_expr
            .function()
            .as_any()
            .downcast_ref::<Identifier>()
            .unwrap();
        assert_eq!(ident.value(), "add");
        assert_eq!(call_expr.arguments().len(), 3);
        let args = call_expr.arguments();
        let int = args[0].as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(int.value(), 1);
        let infix_expr = args[1].as_any().downcast_ref::<InfixExpression>().unwrap();
        let left = infix_expr.left();
        let int = left.as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(int.value(), 2);
        assert_eq!(infix_expr.operator(), "*");
        let right = infix_expr.right();
        let int = right.as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(int.value(), 3);
        let infix_expr = args[2].as_any().downcast_ref::<InfixExpression>().unwrap();
        let left = infix_expr.left();
        let int = left.as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(int.value(), 4);
        assert_eq!(infix_expr.operator(), "+");
        let right = infix_expr.right();
        let int = right.as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(int.value(), 5);
    }
}
