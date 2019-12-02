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
        let pc = self.pc;
        let opcode = self.memory[pc];
        self.pc = pc + 4;

        let in_a = self.memory[pc + 1] as usize;
        let in_b = self.memory[pc + 2] as usize;
        let out = self.memory[pc + 3] as usize;
        match opcode {
            1 => {
                self.memory[out] = self.memory[in_a] + self.memory[in_b];
                true
            }
            2 => {
                self.memory[out] = self.memory[in_a] * self.memory[in_b];
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