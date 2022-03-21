use rlox::chunk::Chunk;
use rlox::opcode::Opcode;
use rlox::vm::Vm;

fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();
    let voff = chunk.add_const(13.0);
    chunk.write_two(Opcode::Constant, voff, 123);
    chunk.write_opcode(Opcode::Return, 123);
    vm.interpret(chunk);
}
