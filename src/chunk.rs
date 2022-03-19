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

    /// addr is u8 because the address will usually
    /// be read from the chunk, which is u8
    pub fn read_val(&self, addr: u8) -> Value {
        self.values[addr as usize]
    }

    pub fn write_opcode(&mut self, opcode: Opcode) {
        self.write(opcode.into());
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_binary(&mut self, o: Opcode, b: u8) {
        self.write_opcode(o);
        self.write(b);
    }

    pub fn add_const(&mut self, val: Value) -> u8 {
        self.values.push(val);
        (self.values.len() - 1).try_into().unwrap()
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
