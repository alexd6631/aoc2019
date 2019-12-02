#[derive(Debug)]
pub struct IntcodeCpu {
    pub memory: Vec<u32>,
    pub pc: usize,
}

impl IntcodeCpu {
    pub fn new(memory: Vec<u32>) -> Self {
        IntcodeCpu { memory, pc: 0 }
    }
}

impl IntcodeCpu {
    fn next(&mut self) -> bool {
        macro_rules! write_indirect {
            ($x: expr, $val: expr) => {{
                let temp = self.memory[$x] as usize;
                self.memory[temp] = $val
            }};
        }

        macro_rules! read_indirect {
            ($x: expr) => { {
                let temp = self.memory[$x] as usize;
                self.memory[temp]
            }};
        }

        macro_rules! arith_instruction {
            ($sym: tt) => {
                write_indirect!(self.pc + 3, read_indirect!(self.pc + 1) $sym read_indirect!(self.pc + 2));
                self.pc += 4;
            }
        }

        let opcode = self.memory[self.pc];
        match opcode {
            1 => {
                arith_instruction!(+);
                true
            }
            2 => {
                arith_instruction!(*);
                true
            }
            99 => { false }
            _ => panic!("Invalid opcode {}", opcode)
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.next() {
                break;
            }
        }
    }
}