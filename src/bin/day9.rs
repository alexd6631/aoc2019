use aoc2019::intcode::{parse_intcode_program, run_with_inputs};

fn main() {
    let program_src = include_str!("../inputs/day9.txt");
    let program = parse_intcode_program(program_src);

    let res = run_with_inputs(program.clone(), vec![1]);
    println!("{:?}", res);

    let res = run_with_inputs(program, vec![2]);
    println!("{:?}", res)

}