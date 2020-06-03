use aoc2019::intcode::{IntcodeCpu, parse_intcode_program, Int};

fn main() {
    let input = include_str!("../inputs/day2.txt");
    solve_part1(&input);
    solve_part2(&input);
}

fn solve_part1(input: &str) {
    let memory: Vec<Int> = parse_intcode_program(input);

    let res = run_with_args(&memory, 12, 2);
    println!("Part 1: {:?}", res)
}

fn solve_part2(input: &str) {
    let rom: Vec<Int> = parse_intcode_program(input);

    if let Some((n, v)) = find_matching_input(&rom, 19_690_720) {
        println!("Part 2: {}", 100 * n + v)
    }
}

fn find_matching_input(rom: &[Int], out: Int) -> Option<(Int, Int)> {
    all_inputs().find(|(noun, verb)| {
        run_with_args(&rom, *noun, *verb) == out
    })
}

fn all_inputs() -> impl Iterator<Item=(Int, Int)> {
    (0..100).flat_map(|noun| (0..100).map(move |verb| {
        (noun, verb)
    }))
}

fn run_with_args(rom: &[Int], noun: Int, verb: Int) -> Int {
    let mut cpu = IntcodeCpu::new(Vec::from(rom));
    cpu.memory[1] = noun;
    cpu.memory[2] = verb;
    cpu.run();
    cpu.memory[0]
}