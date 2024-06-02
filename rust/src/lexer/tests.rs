#[cfg(test)]
mod tests {
    use crate::*;
    use TokenType::*;

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false
}

10 == 10;
10 != 9;"#,
        );

        let tests = [
            Token::new(Let, "let"),
            Token::new(Ident, "five"),
            Token::new(Assign, "="),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "ten"),
            Token::new(Assign, "="),
            Token::new(Int, "10"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "add"),
            Token::new(Assign, "="),
            Token::new(Function, "fn"),
            Token::new(Lparen, "("),
            Token::new(Ident, "x"),
            Token::new(Comma, ","),
            Token::new(Ident, "y"),
            Token::new(Rparen, ")"),
            Token::new(Lbrace, "{"),
            Token::new(Ident, "x"),
            Token::new(Plus, "+"),
            Token::new(Ident, "y"),
            Token::new(Semicolon, ";"),
            Token::new(Rbrace, "}"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "result"),
            Token::new(Assign, "="),
            Token::new(Ident, "add"),
            Token::new(Lparen, "("),
            Token::new(Ident, "five"),
            Token::new(Comma, ","),
            Token::new(Ident, "ten"),
            Token::new(Rparen, ")"),
            Token::new(Semicolon, ";"),
            Token::new(Bang, "!"),
            Token::new(Minus, "-"),
            Token::new(Slash, "/"),
            Token::new(Asterisk, "*"),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(Int, "5"),
            Token::new(Lt, "<"),
            Token::new(Int, "10"),
            Token::new(Gt, ">"),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(If, "if"),
            Token::new(Lparen, "("),
            Token::new(Int, "5"),
            Token::new(Lt, "<"),
            Token::new(Int, "10"),
            Token::new(Rparen, ")"),
            Token::new(Lbrace, "{"),
            Token::new(Return, "return"),
            Token::new(True, "true"),
            Token::new(Semicolon, ";"),
            Token::new(Rbrace, "}"),
            Token::new(Else, "else"),
            Token::new(Lbrace, "{"),
            Token::new(Return, "return"),
            Token::new(False, "false"),
            Token::new(Rbrace, "}"),
            Token::new(Int, "10"),
            Token::new(Eq, "=="),
            Token::new(Int, "10"),
            Token::new(Semicolon, ";"),
            Token::new(Int, "10"),
            Token::new(NotEq, "!="),
            Token::new(Int, "9"),
            Token::new(Semicolon, ";"),
            Token::new(Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token, *test);
        }
    }
}
