use rustox::chunk::Chunk;
use rustox::compiler::Compiler;
use rustox::vm::Vm;

fn main() {
    let source = "(3 + 4) * 5";
    let mut vm = Vm::new();
    let mut comp = Compiler::new(source);
    let chunk = Chunk::new();
    comp.compile(chunk);
    let chunk = comp.take_chunk();
    chunk.disas("Code");
    vm.interpret(chunk);
}
