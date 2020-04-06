use crate::intcode::instruction::{Input, Instruction, Int, decode_instruction, Output};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct IntcodeCpu {
    pub memory: Vec<Int>,
    pub pc: usize,

    pub inputs: VecDeque<Int>,
    pub outputs: Vec<Int>,

    pub is_halted: bool,
    pub relative_base: Int
}

impl IntcodeCpu {
    pub fn new_with_inputs(memory: Vec<Int>, inputs: Vec<Int>) -> Self {
        IntcodeCpu {
            memory,
            pc: 0,
            inputs: VecDeque::from(inputs),
            outputs: Vec::new(),
            is_halted: false,
            relative_base: 0
        }
    }

    pub fn new_with_inputs_and_large_mem(memory_size: usize, program: Vec<Int>, inputs: Vec<Int>) -> Self {
        IntcodeCpu {
            memory: init_memory(memory_size, &program),
            pc: 0,
            inputs: VecDeque::from(inputs),
            outputs: Vec::new(),
            is_halted: false,
            relative_base: 0
        }
    }

    pub fn new(memory: Vec<Int>) -> Self {
        IntcodeCpu::new_with_inputs(memory, Vec::new())
    }
}

fn init_memory(memory_size: usize, program: &Vec<Int>) -> Vec<Int> {
    let mut memory = vec![0 as Int; memory_size];
    memory[..program.len()].copy_from_slice(&program);
    memory
}

impl IntcodeCpu {
    fn input_value(&self, input: Input) -> Int {
        match input {
            Input::Position(p) => self.memory[p],
            Input::Immediate(v) => v,
            Input::Relative(v) => self.memory[(self.relative_base + v) as usize]
        }
    }

    fn output_pos(&self, output: Output) -> usize {
        match output {
            Output::Position(p) => p,
            Output::Relative(v) => (self.relative_base + v) as usize,
        }
    }

    fn next(&mut self) -> bool {
        let instruction = decode_instruction(&self.memory[self.pc..]);
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Add { a, b, out } => {
                let out_pos = self.output_pos(out);
                self.memory[out_pos] = self.input_value(a) + self.input_value(b);
                self.pc += 4;
                true
            }
            Instruction::Mul { a, b, out } => {
                let out_pos = self.output_pos(out);
                self.memory[out_pos] = self.input_value(a) * self.input_value(b);
                self.pc += 4;
                true
            }
            Instruction::In { addr } => {
                if let Some(input) = self.inputs.pop_front() {
                    let out_pos = self.output_pos(addr);
                    self.memory[out_pos] = input;
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
                let out_pos = self.output_pos(out);
                self.memory[out_pos] = if self.input_value(a) < self.input_value(b) {
                    1
                } else { 0 };
                self.pc += 4;
                true
            }
            Instruction::Equals { a, b, out } => {
                let out_pos = self.output_pos(out);
                self.memory[out_pos] = if self.input_value(a) == self.input_value(b) {
                    1
                } else { 0 };
                self.pc += 4;
                true
            },
            Instruction::RelativeBaseOffset { v } => {
                self.relative_base += self.input_value(v);
                self.pc += 2;
                true
            },
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