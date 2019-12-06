use aoc2019::intcode::{parse_intcode_program, run_with_inputs};

fn main() {
    let input = include_str!("day5.txt");
    let memory = parse_intcode_program(input);

    let out = run_with_inputs(memory.clone(), vec![1]);
    println!("Part 1 : {:?}", out.last().expect("No diagnostic code"));

    let out = run_with_inputs(memory.clone(), vec![5]);
    println!("Part 2 : {:?}", out.last().expect("No diagnostic code"));
}