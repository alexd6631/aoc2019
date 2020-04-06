mod instruction;
mod cpu;

pub use cpu::IntcodeCpu;
use instruction::{Input, Instruction, decode_instruction};
pub use instruction::Int;
use std::collections::VecDeque;

pub fn parse_intcode_program(input: &str) -> Vec<Int> {
    input
        .split(',')
        .map(|c| c.parse().expect("Unparseable int"))
        .collect()
}

pub fn run_with_inputs(program: Vec<Int>, inputs: Vec<Int>) -> Vec<Int> {
    let mut cpu = IntcodeCpu::new_with_inputs_and_large_mem(64 * 1024, program, inputs);
    cpu.run();
    cpu.outputs
}

#[cfg(test)]
mod tests {
    use crate::intcode::{parse_intcode_program, run_with_inputs};

    #[test]
    fn test_program() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = parse_intcode_program(input);

        assert_eq!(vec![999], run_with_inputs(program.clone(), vec![4]));
        assert_eq!(vec![1000], run_with_inputs(program.clone(), vec![8]));
        assert_eq!(vec![1001], run_with_inputs(program.clone(), vec![10]));
    }

    #[test]
    fn test_relative_offset() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let program = parse_intcode_program(input);

        let res = run_with_inputs(program, vec![]);
        println!("{:?}", res);

        assert_eq!(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], res)
    }

    #[test]
    fn test_bignum() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let program = parse_intcode_program(input);

        let res = run_with_inputs(program, vec![]);
        println!("{:?}", res);
    }

    #[test]
    fn test_bignum_2() {
        let input = "104,1125899906842624,99";
        let program = parse_intcode_program(input);

        let res = run_with_inputs(program, vec![]);
        assert_eq!(vec![1125899906842624], res);
    }
}

