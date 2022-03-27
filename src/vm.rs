use crate::{chunk::Chunk, opcode::Opcode, value::Value};

pub struct Vm {
    chunk: Option<Chunk>,
    pub stack: Vec<Value>,
    pc: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl Vm {
    pub fn new() -> Self {
        let chunk = None;
        let pc = 0;
        let stack = Vec::new();
        Vm { chunk, pc, stack }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().expect("Chunk is not initialized.")
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.pc = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let opcode: Opcode = self.read().into();
            match opcode {
                Opcode::Invalid => panic!("Invalid instruction."),
                Opcode::Return => {
                    let popped = self.pop();
                    println!("Return: {}", popped);
                    return InterpretResult::Ok
                },
                Opcode::Constant => {
                    let val = self.read_constant();
                    self.push(val);
                },
                Opcode::Negate => {
                    let popped = self.pop();
                    self.push(-popped);
                },
                Opcode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                },
                Opcode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                },
                Opcode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                },
                Opcode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                },
            }
        }
    }

    fn read(&mut self) -> u8 {
        let pc = self.pc;
        let chunk = self.current_chunk();
        let inst = chunk.read(pc);
        self.pc += 1;
        inst
    }

    fn read_value(&mut self, addr: u8) -> Value {
        self.current_chunk().read_value(addr)
    }

    fn read_constant(&mut self) -> Value {
        let addr = self.read();
        self.read_value(addr)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack is empty.")
    }
}