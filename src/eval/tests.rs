#[cfg(test)]
mod tests {
    use crate::{eval::eval, object::Object, Lexer, Parser};

    fn test_eval(s: impl ToString) -> Result<Object, String> {
        let l = Lexer::new(s.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program()?;
        Ok(eval(Box::new(program)))
    }

    #[test]
    fn test_eval_integer_expression() {
        let inputs = vec![("5", 5), ("10", 10)];
        for (input, expected) in inputs {
            let _ = test_eval(input).unwrap();
            // assert_eq!(obj, expected);
        }
    }
}
