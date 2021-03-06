use rustox::chunk::Chunk;
use rustox::compiler::Compiler;
use rustox::vm::Vm;

fn main() {
    let source = r#"var beverage = "cafe au lait";
    var breakfast = "beignets with " + beverage;
    print breakfast;
    "#;
    let mut vm = Vm::new();
    let mut comp = Compiler::new(source);
    let chunk = Chunk::new();
    let had_error = comp.compile(chunk);
    if had_error {
        return;
    }

    let chunk = comp.take_chunk();
    chunk.disas("Code");
    vm.interpret(chunk);
}
