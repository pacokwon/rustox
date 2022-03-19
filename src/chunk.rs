use crate::opcode::Opcode;

type Value = f64;

pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        let code = Vec::new();
        let values = Vec::new();

        Chunk { code, values }
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.code[addr]
    }

    pub fn read_opcode(&self, addr: usize) -> Opcode {
        self.code[addr].into()
    }

    pub fn write_opcode(&mut self, opcode: Opcode) {
        self.write(opcode.into());
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_two_opcodes(&mut self, o1: Opcode, o2: Opcode) {
        self.write_opcode(o1);
        self.write_opcode(o2);
    }

    pub fn write_two(&mut self, b1: u8, b2: u8) {
        self.code.push(b1);
        self.code.push(b2);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
