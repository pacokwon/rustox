use crate::chunk::Chunk;

pub fn disas(chunk: &Chunk, name: &str) {
    println!("===== {} =====", name);

    let mut offset = 0;
    while offset < chunk.len() {
        offset = disas_inst(chunk, offset);
    }
}

pub fn disas_inst(chunk: &Chunk, offset: usize) -> usize {
    print!("{addr:<width$} ", addr=offset, width=4);

    let opcode = chunk.read_opcode(offset);

    println!("{:?}", opcode);
    offset + opcode.len()
}
