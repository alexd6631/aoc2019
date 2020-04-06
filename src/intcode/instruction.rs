pub type Int = i64;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Add {
        a: Input,
        b: Input,
        out: Output,
    },
    Mul {
        a: Input,
        b: Input,
        out: Output,
    },
    In {
        addr: Output
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
        out: Output,
    },
    Equals {
        a: Input,
        b: Input,
        out: Output,
    },
    RelativeBaseOffset {
        v: Input
    },
    Halt
}

#[derive(Debug, Eq, PartialEq)]
pub enum Input {
    Position(usize),
    Immediate(Int),
    Relative(Int)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    Position(usize),
    Relative(Int)
}

pub fn decode_instruction(ptr: &[Int]) -> Instruction {
    let code = ptr[0];
    let opcode_int = code % 100;

    let modes = [
        (code / 100) % 10,
        (code / 1000) % 10,
        (code / 10000) % 10,
    ];

    macro_rules! decode_input { ($n: expr) => {
        match modes[$n] {
            0 => Input::Position(ptr[$n + 1] as usize),
            1 => Input::Immediate(ptr[$n + 1]),
            2 => Input::Relative(ptr[$n + 1]),
            _ => panic!("invalid addressing mode")
        }

        };
    }


    macro_rules! decode_output { ($n: expr) => {
        match modes[$n] {
            0 => Output::Position(ptr[$n + 1] as usize),
            2 => Output::Relative(ptr[$n + 1]),
            _ => panic!("invalid addressing mode")
        }

        };
    }

    match opcode_int {
        1 => Instruction::Add {
            a: decode_input!(0),
            b: decode_input!(1),
            out: decode_output!(2),
        },
        2 => Instruction::Mul {
            a: decode_input!(0),
            b: decode_input!(1),
            out: decode_output!(2),
        },
        3 => Instruction::In {
            addr: decode_output!(0)
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
            out: decode_output!(2),
        },
        8 => Instruction::Equals {
            a: decode_input!(0),
            b: decode_input!(1),
            out: decode_output!(2),
        },
        9 => Instruction::RelativeBaseOffset {
            v: decode_input!(0)
        },
        99 => Instruction::Halt,
        _ => panic!("Unsupported opcode {}", opcode_int)
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::instruction::{decode_instruction, Instruction, Input, Output};

    #[test]
    fn test_decode() {
        let mem = [1002, 4, 3, 4];
        let instr = decode_instruction(&mem[0..]);

        assert_eq!(instr, Instruction::Mul {
            a: Input::Position(4),
            b: Input::Immediate(3),
            out: Output::Position(4),
        })
    }
}