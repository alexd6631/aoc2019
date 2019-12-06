type Int = i32;

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

#[derive(Debug, Eq, PartialEq)]
enum Input {
    Position(usize),
    Immediate(Int),
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Add {
        a: Input,
        b: Input,
        out: usize,
    },
    Mul {
        a: Input,
        b: Input,
        out: usize,
    },
    In {
        addr: usize
    },
    Out {
        addr: Input
    },
    JumpIfTrue {
        v: Input,
        addr: Input,
    },
    JumpIfFalse {
        v: Input,
        addr: Input,
    },
    LessThan {
        a: Input,
        b: Input,
        out: usize,
    },
    Equals {
        a: Input,
        b: Input,
        out: usize,
    },
    Halt,
}

fn decode_instruction(ptr: &[Int]) -> Instruction {
    let code = ptr[0];
    let opcode_int = code % 100;
    let modes = [
        (code / 100) % 10 == 1,
        (code / 1000) % 10 == 1,
        (code / 10000) % 10 == 1,
    ];

    macro_rules! decode_input { ($n: expr) => {
        if modes[$n] { Input::Immediate(ptr[$n + 1])} else { Input::Position(ptr[$n + 1] as usize) }
        };
    }
    match opcode_int {
        1 => Instruction::Add {
            a: decode_input!(0),
            b: decode_input!(1),
            out: ptr[3] as usize,
        },
        2 => Instruction::Mul {
            a: decode_input!(0),
            b: decode_input!(1),
            out: ptr[3] as usize,
        },
        3 => Instruction::In {
            addr: ptr[1] as usize
        },
        4 => Instruction::Out {
            addr: decode_input!(0)
        },
        5 => Instruction::JumpIfTrue {
            v: decode_input!(0),
            addr: decode_input!(1)
        },
        6 => Instruction::JumpIfFalse {
            v: decode_input!(0),
            addr: decode_input!(1)
        },
        7 => Instruction::LessThan {
            a: decode_input!(0),
            b: decode_input!(1),
            out: ptr[3] as usize,
        },
        8 => Instruction::Equals {
            a: decode_input!(0),
            b: decode_input!(1),
            out: ptr[3] as usize,
        },
        99 => Instruction::Halt,
        _ => panic!("Unsupported opcode {}", opcode_int)
    }
}

pub fn run_with_inputs(program: Vec<Int>, inputs: Vec<Int>) -> Vec<Int> {
    let mut cpu = IntcodeCpu::new_with_inputs(program, inputs);
    cpu.run();
    cpu.outputs
}

#[cfg(test)]
mod tests {
    use crate::intcode::{decode_instruction, Instruction, Input, parse_intcode_program, run_with_inputs};

    #[test]
    fn test_decode() {
        let mem = [1002, 4, 3, 4];
        let instr = decode_instruction(&mem[0..]);

        assert_eq!(instr, Instruction::Mul {
            a: Input::Position(4),
            b: Input::Immediate(3),
            out: 4,
        })
    }

    #[test]
    fn test_program() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = parse_intcode_program(input);

        assert_eq!(vec![999], run_with_inputs(program.clone(), vec![4]));
        assert_eq!(vec![1000], run_with_inputs(program.clone(), vec![8]));
        assert_eq!(vec![1001], run_with_inputs(program.clone(), vec![10]));
    }
}

pub fn parse_intcode_program(input: &str) -> Vec<Int> {
    input
        .split(',')
        .map(|c| c.parse().expect("Unparseable int"))
        .collect()
}