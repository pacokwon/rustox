use crate::opcode::Opcode;

type Value = f64;

pub struct Chunk {
    code: Vec<u8>,
    pub lines: Vec<u32>,
    values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        let code = Vec::new();
        let lines = Vec::new();
        let values = Vec::new();

        Chunk { code, lines, values }
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.code[addr]
    }

    pub fn read_opcode(&self, addr: usize) -> Opcode {
        self.code[addr].into()
    }

    /// addr is u8 because the address will usually
    /// be read from the chunk, which is u8
    pub fn read_val(&self, addr: u8) -> Value {
        self.values[addr as usize]
    }

    pub fn write_opcode(&mut self, opcode: Opcode, line: u32) {
        self.write(opcode.into(), line);
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_binary(&mut self, o: Opcode, b: u8, line: u32) {
        self.write_opcode(o, line);
        self.write(b, line);
    }

    pub fn add_const(&mut self, val: Value) -> u8 {
        self.values.push(val);
        (self.values.len() - 1).try_into().unwrap()
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
