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
            return self.error_token();
        }

        self.start = self.current;
        let c = self.advance();

        let mut eq_lookahead = |eq, ne| {
            let tt = {
                if self.consume_eq('=') { eq } else { ne }
            };
            self.make_token(tt)
        };

        match c {
            '(' => return self.make_token(LeftParen),
            ')' => return self.make_token(RightParen),
            '{' => return self.make_token(LeftBrace),
            '}' => return self.make_token(RightBrace),
            ',' => return self.make_token(Comma),
            '.' => return self.make_token(Dot),
            '-' => return self.make_token(Minus),
            '+' => return self.make_token(Plus),
            ';' => return self.make_token(Semicolon),
            '*' => return self.make_token(Star),
            '/' => return self.make_token(Slash),
            '!' => return eq_lookahead(BangEqual, Equal),
            '=' => return eq_lookahead(EqualEqual, Equal),
            '>' => return eq_lookahead(GreaterEqual, Greater),
            '<' => return eq_lookahead(LesserEqual, Lesser),
            '/' => {
                if self.consume_eq('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    return self.make_token(Slash);
                }
            }
            '"' => return self.string(),
            d if d.is_digit(10) => return self.number(),
            a if a.is_alphabetic() => return self.ident_and_keyword(),
            _ => (),
        };

        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source_vec[self.current];
        self.current += 1;
        c
    }

    fn consume_eq(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            false
        } else {
            true
        }
    }

    fn peek(&self) -> char {
        return self.source_vec[self.current];
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source_vec[self.current])
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                '\r' | ' ' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                },
                '/' => if let Some('/') = self.peek_next() {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                },
                _ => break,
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' {
            self.advance();
        }
        let token = self.make_token(TokenType::String);
        self.advance();
        token
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let num = self.source[self.start..self.current].parse::<f64>().unwrap();
        Token {
            token_type: TokenType::Number(num),
            lexeme: self.source[self.start..self.current].to_owned(),
            line: self.line,
        }
    }

    fn ident_and_keyword(&mut self) -> Token {
        let is_alpha = |c| ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_';
        let is_alphanum = |c| ('0' <= c && c <= '9') || is_alpha(c);

        while is_alphanum(self.peek()) {
            self.advance();
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
            Some('o') => compare_rest(1, "r", Nil),
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
                Some('u') => compare_rest(2, "e", True),
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

    fn error_token(&self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foobar() {
        assert_eq!(3, 3);
    }
}
