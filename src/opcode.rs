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
        let lookup_tbl = [Return, Constant];
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
