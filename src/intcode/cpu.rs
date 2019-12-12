use crate::intcode::instruction::{Input, Instruction, Int, decode_instruction};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct IntcodeCpu {
    pub memory: Vec<Int>,
    pub pc: usize,

    pub inputs: VecDeque<Int>,
    pub outputs: Vec<Int>,

    pub is_halted: bool
}

impl IntcodeCpu {
    pub fn new_with_inputs(memory: Vec<Int>, inputs: Vec<Int>) -> Self {
        IntcodeCpu {
            memory,
            pc: 0,
            inputs: VecDeque::from(inputs),
            outputs: Vec::new(),
            is_halted: false
        }
    }

    pub fn new(memory: Vec<Int>) -> Self {
        IntcodeCpu {
            memory,
            pc: 0,
            inputs: VecDeque::new(),
            outputs: Vec::new(),
            is_halted: false
        }
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
                if let Some(input) = self.inputs.pop_front() {
                    self.memory[addr] = input;
                    self.pc += 2;
                    true
                } else {
                    false
                }
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
            Instruction::Halt => {
                self.is_halted = true;
                false
            },
        }
    }

    pub fn run(&mut self) {
        while self.next() {}
    }
}