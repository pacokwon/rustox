use crate::chunk::Chunk;
use crate::opcode::Opcode;

impl Chunk {
    pub fn disas(&self, name: &str) {
        println!("===== {} =====", name);

        let mut offset = 0;
        while offset < self.len() {
            offset = self.disas_inst(offset);
        }
    }

    pub fn disas_inst(&self, offset: usize) -> usize {
        print!("{addr:0>width$} ", addr=offset, width=4);

        if offset > 0 && self.lines[offset - 1] == self.lines[offset] {
            print!("{:>4} ", "|");
        } else {
            print!("{:>4} ", self.lines[offset]);
        }

        let opcode = self.read_opcode(offset);
        self.disas_opcode(opcode, offset);
        offset + opcode.len()
    }

    pub fn disas_opcode(&self, opcode: Opcode, offset: usize) {
        use Opcode::*;
        match opcode {
            Invalid | Return => println!("{:?}", opcode),
            Constant => {
                let voff = self.read(offset + 1);
                let constant = self.read_val(voff);
                println!("{:<16} {:<4} '{}'", format!("{:?}", opcode), voff, constant);
            },
        }
    }
}
