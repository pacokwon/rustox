use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub struct Parser<'src> {
    pub scanner: Scanner<'src>,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panicking: bool,
}

enum ErrorPoint {
    Current,
    Previous,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut scanner = Scanner::new(source);
        let placeholder = Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            line: 1,
        };
        let current = scanner.scan_token();

        Parser {
            scanner,
            current,
            previous: placeholder,
            had_error: false,
            panicking: false,
        }
    }

    pub fn advance(&mut self) {
        // self.previous = self.current
        // we can't do this in safe Rust code, so we swap these two
        std::mem::swap(&mut self.current, &mut self.previous);

        loop {
            self.current = self.scanner.scan_token();

            match self.current.token_type {
                TokenType::Error => self.error_at(ErrorPoint::Current, "Encountered error token."),
                _ => break,
            };
        }
    }

    pub fn synchronize(&mut self) {
        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.current.token_type {
                TokenType::Class |
                TokenType::For |
                TokenType::Fun |
                TokenType::If |
                TokenType::Print |
                TokenType::Return |
                TokenType::Var |
                TokenType::While => return,
                _ => (),
            }

            self.advance();
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: &'static str) {
        if self.current.token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    pub fn check(&mut self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    pub fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            false
        } else {
            self.advance();
            true
        }
    }

    pub fn error_at_current(&mut self, message: &'static str) {
        self.error_at(ErrorPoint::Current, message);
    }

    pub fn error(&mut self, message: &'static str) {
        self.error_at(ErrorPoint::Previous, message);
    }

    fn error_at(&mut self, location: ErrorPoint, message: &'static str) {
        if self.panicking {
            return;
        }
        self.panicking = true;

        let token = match location {
            ErrorPoint::Current => &mut self.current,
            ErrorPoint::Previous => &mut self.previous,
        };

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", token.lexeme),
        };

        eprintln!(": {}", message);
        self.had_error = true;
    }
}
