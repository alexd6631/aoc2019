mod instruction;
mod cpu;

pub use cpu::IntcodeCpu;
use instruction::{Input, Instruction, Int, decode_instruction};

pub fn parse_intcode_program(input: &str) -> Vec<Int> {
    input
        .split(',')
        .map(|c| c.parse().expect("Unparseable int"))
        .collect()
}

pub fn run_with_inputs(program: Vec<Int>, inputs: Vec<Int>) -> Vec<Int> {
    let mut cpu = IntcodeCpu::new_with_inputs(program, inputs);
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
}

