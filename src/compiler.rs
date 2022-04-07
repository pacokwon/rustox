use crate::chunk::Chunk;
use crate::opcode::{Opcode, Precedence};
use crate::parser::Parser;
use crate::token::TokenType;
use crate::value::Value;

pub struct Compiler<'src> {
    parser: Parser<'src>,
    chunk: Option<Chunk>,
    rules: Vec<ParseRule<'src>>,
}

struct ParseRule<'src> {
    prefix: fn(&mut Compiler<'src>),
    infix: fn(&mut Compiler<'src>),
    precedence: Precedence,
}

fn init_rules<'src>() -> Vec<ParseRule<'src>> {
    use TokenType::*;

    let mut rules = Vec::with_capacity(41);
    let mut set = |token, prefix, infix, precedence| {
        assert_eq!(token as usize, rules.len());
        rules.push(ParseRule {
            prefix,
            infix,
            precedence,
        });
    };

    set(
        LeftParen,
        Compiler::grouping,
        Compiler::skip,
        Precedence::None,
    );
    set(RightParen, Compiler::skip, Compiler::skip, Precedence::None);
    set(LeftBrace, Compiler::skip, Compiler::skip, Precedence::None);
    set(RightBrace, Compiler::skip, Compiler::skip, Precedence::None);
    set(Comma, Compiler::skip, Compiler::skip, Precedence::None);
    set(Dot, Compiler::skip, Compiler::skip, Precedence::None);
    set(Minus, Compiler::unary, Compiler::binary, Precedence::Term);
    set(Plus, Compiler::skip, Compiler::binary, Precedence::Term);
    set(Semicolon, Compiler::skip, Compiler::skip, Precedence::None);
    set(Slash, Compiler::skip, Compiler::binary, Precedence::Factor);
    set(Star, Compiler::skip, Compiler::binary, Precedence::Factor);

    set(Bang, Compiler::unary, Compiler::skip, Precedence::None);
    set(
        BangEqual,
        Compiler::skip,
        Compiler::binary,
        Precedence::Equality,
    );
    set(Equal, Compiler::skip, Compiler::skip, Precedence::None);
    set(
        EqualEqual,
        Compiler::skip,
        Compiler::binary,
        Precedence::Equality,
    );
    set(
        Greater,
        Compiler::skip,
        Compiler::binary,
        Precedence::Comparison,
    );
    set(
        GreaterEqual,
        Compiler::skip,
        Compiler::binary,
        Precedence::Comparison,
    );
    set(
        Lesser,
        Compiler::skip,
        Compiler::binary,
        Precedence::Comparison,
    );
    set(
        LesserEqual,
        Compiler::skip,
        Compiler::binary,
        Precedence::Comparison,
    );

    set(Identifier, Compiler::skip, Compiler::skip, Precedence::None);
    set(String, Compiler::string, Compiler::skip, Precedence::None);
    set(Number, Compiler::number, Compiler::skip, Precedence::None);

    set(And, Compiler::skip, Compiler::binary, Precedence::And);
    set(Assert, Compiler::skip, Compiler::skip, Precedence::None);
    set(Class, Compiler::skip, Compiler::skip, Precedence::None);
    set(Else, Compiler::skip, Compiler::skip, Precedence::None);
    set(False, Compiler::literal, Compiler::skip, Precedence::None);
    set(For, Compiler::skip, Compiler::skip, Precedence::None);
    set(Fun, Compiler::skip, Compiler::skip, Precedence::None);
    set(If, Compiler::skip, Compiler::skip, Precedence::None);
    set(Nil, Compiler::literal, Compiler::skip, Precedence::None);
    set(Or, Compiler::skip, Compiler::binary, Precedence::Or);
    set(Print, Compiler::skip, Compiler::skip, Precedence::None);
    set(Return, Compiler::skip, Compiler::skip, Precedence::None);
    set(Super, Compiler::skip, Compiler::skip, Precedence::None);
    set(This, Compiler::skip, Compiler::skip, Precedence::None);
    set(True, Compiler::literal, Compiler::skip, Precedence::None);
    set(Var, Compiler::skip, Compiler::skip, Precedence::None);
    set(While, Compiler::skip, Compiler::skip, Precedence::None);

    set(Error, Compiler::skip, Compiler::skip, Precedence::None);
    set(Eof, Compiler::skip, Compiler::skip, Precedence::None);

    rules
}

impl<'src> Compiler<'src> {
    pub fn new(source: &'src str) -> Self {
        let parser = Parser::new(source);
        let chunk = None;
        let rules = init_rules();
        Compiler {
            parser,
            chunk,
            rules,
        }
    }

    pub fn compile(&mut self, chunk: Chunk) -> bool {
        self.set_chunk(chunk);

        while !self.parser.match_token(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();
        self.parser.had_error
    }

    pub fn set_chunk(&mut self, chunk: Chunk) {
        self.chunk = Some(chunk);
    }

    pub fn take_chunk(&mut self) -> Chunk {
        self.chunk.take().expect("Cannot take empty chunk")
    }

    fn compiling_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().expect("Compiling chunk is not set.")
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.line;
        let chunk = self.compiling_chunk();
        chunk.write(byte, line);
    }

    fn emit_opcode(&mut self, opcode: Opcode) {
        let line = self.parser.previous.line;
        let chunk = self.compiling_chunk();
        chunk.write_opcode(opcode, line);
    }

    fn emit_bytes(&mut self, first: u8, second: u8) {
        self.emit_byte(first);
        self.emit_byte(second);
    }

    fn emit_two(&mut self, first: Opcode, second: u8) {
        self.emit_opcode(first);
        self.emit_byte(second);
    }

    fn emit_const(&mut self, value: Value) {
        self.emit_opcode(Opcode::Constant);
        let value_index = self.make_const(value);
        self.emit_byte(value_index);
    }

    fn make_const(&mut self, value: Value) -> u8 {
        let chunk = self.compiling_chunk();
        chunk.add_const(value)
    }

    fn end_compiler(&mut self) {
        self.emit_opcode(Opcode::Return);
    }

    fn get_rule(&self, ttype: TokenType) -> &ParseRule<'src> {
        &self.rules[ttype as usize]
    }

    fn parse_precedence(&mut self, prec: Precedence) {
        self.parser.advance();
        let prefix_rule = self.get_rule(self.parser.previous.token_type).prefix;
        if prefix_rule as usize == Self::skip as usize {
            self.parser.error("Expect expression.");
            return;
        }

        prefix_rule(self);

        while prec <= self.get_rule(self.parser.current.token_type).precedence {
            // we keep parsing if higher (or equal) parsing rules keep coming
            // ex> 3 + 4 * 2
            //       ^--
            self.parser.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type).infix;
            assert_ne!(infix_rule as usize, Self::skip as usize);
            infix_rule(self);
        }
    }

    /// a program is a sequence of declarations
    /// declaration <- classDecl
    ///                varDecl
    ///                funDecl
    ///                statement
    fn declaration(&mut self) {
        self.statement();

        if self.parser.panicking {
            self.parser.synchronize();
        }
    }

    /// statement <- exprStmt
    ///              printStmt
    fn statement(&mut self) {
        if self.parser.match_token(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::Semicolon, "Expected ';' after print statement.");
        self.emit_opcode(Opcode::Print);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::Semicolon, "Expected ';' after expression.");
    }

    fn number(&mut self) {
        let value = self
            .parser
            .previous
            .lexeme
            .parse::<f64>()
            .expect("Expected number.");

        self.emit_const(Value::Number(value));
    }

    fn string(&mut self) {
        let lexeme = &self.parser.previous.lexeme;
        let length = lexeme.len();
        let value = &lexeme[1..length - 1].to_owned();

        self.emit_const(Value::String(value.to_owned()));
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let op_type = self.parser.previous.token_type;

        self.expression();

        match op_type {
            TokenType::Minus => self.emit_opcode(Opcode::Negate),
            TokenType::Bang => self.emit_opcode(Opcode::Not),
            _ => panic!("Invalid unary operator token {:?}", op_type),
        }
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous.token_type;
        let next_prec = self.get_rule(op_type).precedence.next();
        self.parse_precedence(next_prec);

        match op_type {
            TokenType::Plus => self.emit_opcode(Opcode::Add),
            TokenType::Minus => self.emit_opcode(Opcode::Subtract),
            TokenType::Star => self.emit_opcode(Opcode::Multiply),
            TokenType::Slash => self.emit_opcode(Opcode::Divide),
            TokenType::EqualEqual => self.emit_opcode(Opcode::Equal),
            TokenType::BangEqual => {
                self.emit_opcode(Opcode::Equal);
                self.emit_opcode(Opcode::Not);
            }
            TokenType::Greater => self.emit_opcode(Opcode::Greater),
            TokenType::Lesser => self.emit_opcode(Opcode::Lesser),
            TokenType::GreaterEqual => {
                self.emit_opcode(Opcode::Lesser);
                self.emit_opcode(Opcode::Not);
            }
            TokenType::LesserEqual => {
                self.emit_opcode(Opcode::Greater);
                self.emit_opcode(Opcode::Not);
            }
            TokenType::And => self.emit_opcode(Opcode::And),
            TokenType::Or => self.emit_opcode(Opcode::Or),
            _ => panic!("Invalid binary operator token {:?}", op_type),
        };
    }

    fn literal(&mut self) {
        let op_type = self.parser.previous.token_type;
        match op_type {
            TokenType::Nil => self.emit_opcode(Opcode::Nil),
            TokenType::True => self.emit_opcode(Opcode::True),
            TokenType::False => self.emit_opcode(Opcode::False),
            _ => panic!("Invalid literal token {:?}", op_type),
        }
    }

    /// dummy parse function for doing nothing
    fn skip(&mut self) {}
}
