pub type Int = i32;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
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

#[derive(Debug, Eq, PartialEq)]
pub enum Input {
    Position(usize),
    Immediate(Int),
}

pub fn decode_instruction(ptr: &[Int]) -> Instruction {
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

#[cfg(test)]
mod tests {
    use crate::intcode::instruction::{decode_instruction, Instruction, Input};

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
}