#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Return = 0,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Invalid = 255,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        (v as usize).into()
    }
}

impl From<usize> for Opcode {
    fn from(v: usize) -> Self {
        use Opcode::*;
        let lookup_tbl = [
            Return,
            Constant,
            Negate,
            Add,
            Subtract,
            Multiply,
            Divide,
        ];
        if v < lookup_tbl.len() {
            lookup_tbl[v]
        } else {
            Invalid
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Opcode {
    pub fn len(&self) -> usize {
        use Opcode::*;

        match self {
            Invalid => 0,
            Return | Negate | Add | Subtract | Multiply | Divide => 1,
            Constant => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u8> for Precedence {
    fn from(v: u8) -> Self {
        (v as usize).into()
    }
}

impl From<usize> for Precedence {
    fn from(v: usize) -> Self {
        let lookup_tbl = [
            Precedence::None,
            Precedence::Assignment,
            Precedence::Or,
            Precedence::And,
            Precedence::Equality,
            Precedence::Comparison,
            Precedence::Term,
            Precedence::Factor,
            Precedence::Unary,
            Precedence::Call,
            Precedence::Primary,
        ];
        if v < lookup_tbl.len() {
            lookup_tbl[v]
        } else {
            Precedence::Primary
        }
    }
}

impl Precedence {
    pub fn next(&self) -> Precedence {
        let prec_num = *self as usize;
        (prec_num + 1).into()
    }
}
