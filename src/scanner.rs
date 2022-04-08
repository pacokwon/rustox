use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'src> {
    source: &'src str,
    // I could replace this vector with a window iterator.
    // but I'm keeping it for simplicity's sake,
    // since chars() does not provide a windows() function
    source_vec: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
            source,
            source_vec: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();
        if self.is_at_end() {
            return Token {
                token_type: Eof,
                lexeme: "".to_owned(),
                line: self.line,
            };
        }

        self.start = self.current;
        let c = self.advance();

        let mut eq_lookahead = |eq, ne| {
            let tt = {
                if self.consume_eq('=') {
                    eq
                } else {
                    ne
                }
            };
            self.make_token(tt)
        };

        if c.is_none() {
            panic!("Not at EOF, but advance() returned None.");
        }

        match c.unwrap() {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            ';' => self.make_token(Semicolon),
            '*' => self.make_token(Star),
            '/' => self.make_token(Slash),
            '!' => eq_lookahead(BangEqual, Bang),
            '=' => eq_lookahead(EqualEqual, Equal),
            '>' => eq_lookahead(GreaterEqual, Greater),
            '<' => eq_lookahead(LesserEqual, Lesser),
            '"' => self.string(),
            d if d.is_digit(10) => self.number(),
            a if a.is_alphabetic() => self.ident_and_keyword(),
            _ => self.error_token("Invalid token."),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.current += 1;
        }
        c
    }

    fn consume_eq(&mut self, c: char) -> bool {
        if let Some(p) = self.peek() {
            if c == p {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        if self.current < self.source_vec.len() {
            Some(self.source_vec[self.current])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source_vec[self.current + 1])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '\r' | ' ' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                }
                '/' => {
                    if let Some('/') = self.peek_next() {
                        while let Some(c) = self.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn string(&mut self) -> Token {
        loop {
            match self.peek() {
                Some(c) => {
                    if c == '"' {
                        break;
                    } else {
                        self.advance();
                    }
                }
                None => return self.error_token("Reached Eof while scanning string."),
            }
        }

        self.advance();
        let token = self.make_token(TokenType::String);
        token
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        if let Some('.') = self.peek() {
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        Token {
            token_type: TokenType::Number,
            lexeme: self.source[self.start..self.current].to_owned(),
            line: self.line,
        }
    }

    fn ident_and_keyword(&mut self) -> Token {
        let is_alpha = |c| ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_';
        let is_alphanum = |c| ('0' <= c && c <= '9') || is_alpha(c);

        while let Some(c) = self.peek() {
            if is_alphanum(c) {
                self.advance();
            } else {
                break;
            }
        }

        let token_type = self.ident_keyword_type();
        self.make_token(token_type)
    }

    fn ident_keyword_type(&self) -> TokenType {
        use TokenType::*;

        let ident = &self.source[self.start..self.current];
        let mut it = ident.chars();
        let compare_rest = |offset, rest, tt| {
            if &ident[offset..] == rest {
                tt
            } else {
                Identifier
            }
        };

        // List of keywords in Lox
        // and, assert,
        // class,
        // else,
        // false, for, fun,
        // if,
        // nil,
        // or,
        // print,
        // return,
        // super,
        // this, true,
        // var,
        // while,
        match it.next() {
            Some('c') => compare_rest(1, "lass", Class),
            Some('e') => compare_rest(1, "lse", Else),
            Some('i') => compare_rest(1, "f", If),
            Some('n') => compare_rest(1, "il", Nil),
            Some('o') => compare_rest(1, "r", Or),
            Some('p') => compare_rest(1, "rint", Print),
            Some('r') => compare_rest(1, "eturn", Return),
            Some('s') => compare_rest(1, "uper", Super),
            Some('v') => compare_rest(1, "ar", Var),
            Some('w') => compare_rest(1, "hile", While),
            Some('a') => match it.next() {
                Some('n') => compare_rest(2, "d", And),
                Some('s') => compare_rest(2, "sert", Assert),
                _ => Identifier,
            },
            Some('f') => match it.next() {
                Some('a') => compare_rest(2, "lse", False),
                Some('o') => compare_rest(2, "r", For),
                Some('u') => compare_rest(2, "n", Fun),
                _ => Identifier,
            },
            Some('t') => match it.next() {
                Some('h') => compare_rest(2, "is", This),
                Some('r') => compare_rest(2, "ue", True),
                _ => Identifier,
            },
            _ => Identifier,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.source[self.start..self.current].to_owned(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message.to_owned(),
            line: self.line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    fn scan_tokens(code: &str) -> Vec<Token> {
        let mut sc = Scanner::new(code);
        let mut v = Vec::new();
        while !sc.is_at_end() {
            v.push(sc.scan_token())
        }
        println!("{:?}", v);
        v
    }

    fn compare(result: Vec<Token>, expected: Vec<TokenType>) {
        assert_eq!(result.len(), expected.len());
        let compare_tt = |tt1, tt2| assert_eq!(tt1, tt2);

        result
            .into_iter()
            .zip(expected)
            .for_each(|(r, e)| compare_tt(r.token_type, e))
    }

    fn test_code(code: &str, expected: Vec<TokenType>) {
        let result = scan_tokens(code);
        compare(result, expected);
    }

    #[test]
    fn variable() {
        let code = "var a = 1;";
        let expected = vec![Var, Identifier, Equal, Number, Semicolon];
        test_code(code, expected);
    }

    #[test]
    fn variable_long() {
        let code = "var a = 1;\n\
                    var b = 2;\n\
                    print a;";
        let expected = vec![
            Var,
            Identifier,
            Equal,
            Number,
            Semicolon,
            Var,
            Identifier,
            Equal,
            Number,
            Semicolon,
            Print,
            Identifier,
            Semicolon,
        ];
        test_code(code, expected);
    }

    #[test]
    fn operators() {
        let code = "+ - * / < > = ! <= >= == !=";
        let expected = vec![
            Plus,
            Minus,
            Star,
            Slash,
            Lesser,
            Greater,
            Equal,
            Bang,
            LesserEqual,
            GreaterEqual,
            EqualEqual,
            BangEqual,
        ];
        test_code(code, expected);
    }

    #[test]
    fn symbols() {
        let code = "} { ) ( . , ;";
        let expected = vec![
            RightBrace, LeftBrace, RightParen, LeftParen, Dot, Comma, Semicolon,
        ];
        test_code(code, expected);
    }

    #[test]
    fn literals() {
        let code = "\"Hello World\" 3.1415";
        let expected = vec![String, Number];
        test_code(code, expected);
    }

    #[test]
    fn comment() {
        let code = "// this is a comment\n\
                    / \"this is not a comment\"";
        let expected = vec![Slash, String];
        test_code(code, expected);
    }

    #[test]
    fn string_eof() {
        let code = "\"weird string";
        let expected = vec![Error];
        test_code(code, expected);
    }

    #[test]
    fn comparison() {
        let code = "3 < 4";
        let expected = vec![Number, Lesser, Number];
        test_code(code, expected);
    }

    #[test]
    fn logical() {
        let code = "true and false";
        let expected = vec![True, And, False];
        test_code(code, expected);
    }

    #[test]
    fn logical2() {
        let code = "true or true and false";
        let expected = vec![True, Or, True, And, False];
        test_code(code, expected);
    }

    #[test]
    fn global() {
        let code = r#"var beverage = "cafe au lait";"#;
        let expected = vec![Var, Identifier, Equal, String, Semicolon];
        test_code(code, expected);
    }
}
