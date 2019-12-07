use crate::intcode::instruction::{Input, Instruction, Int, decode_instruction};

#[derive(Debug)]
pub struct IntcodeCpu {
    pub memory: Vec<Int>,
    pub pc: usize,

    pub inputs: Vec<Int>,
    pub outputs: Vec<Int>,
}

impl IntcodeCpu {
    pub fn new_with_inputs(memory: Vec<Int>, inputs: Vec<Int>) -> Self {
        IntcodeCpu { memory, pc: 0, inputs, outputs: Vec::new() }
    }

    pub fn new(memory: Vec<Int>) -> Self {
        IntcodeCpu { memory, pc: 0, inputs: Vec::new(), outputs: Vec::new() }
    }
}

impl IntcodeCpu {
    fn input_value(&self, input: Input) -> Int {
        match input {
            Input::Position(p) => self.memory[p],
            Input::Immediate(v) => v,
        }
    }

    fn next(&mut self) -> bool {
        let instruction = decode_instruction(&self.memory[self.pc..]);
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Add { a, b, out } => {
                self.memory[out] = self.input_value(a) + self.input_value(b);
                self.pc += 4;
                true
            }
            Instruction::Mul { a, b, out } => {
                self.memory[out] = self.input_value(a) * self.input_value(b);
                self.pc += 4;
                true
            }
            Instruction::In { addr } => {
                self.memory[addr] = self.inputs.pop().expect("Input buffer is empty");
                self.pc += 2;
                true
            }
            Instruction::Out { addr } => {
                self.outputs.push(self.input_value(addr));
                self.pc += 2;
                true
            }
            Instruction::JumpIfTrue { v, addr } => {
                if self.input_value(v) != 0 {
                    self.pc = self.input_value(addr) as usize;
                } else {
                    self.pc += 3;
                }
                true
            }
            Instruction::JumpIfFalse { v, addr } => {
                if self.input_value(v) == 0 {
                    self.pc = self.input_value(addr) as usize;
                } else {
                    self.pc += 3;
                }
                true
            }
            Instruction::LessThan { a, b, out } => {
                self.memory[out] = if self.input_value(a) < self.input_value(b) {
                    1
                } else { 0 };
                self.pc += 4;
                true
            }
            Instruction::Equals { a, b, out } => {
                self.memory[out] = if self.input_value(a) == self.input_value(b) {
                    1
                } else { 0 };
                self.pc += 4;
                true
            }
            Instruction::Halt => false,
        }
    }

    pub fn run(&mut self) {
        while self.next() {}
    }
}