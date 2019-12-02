use aoc2019::intcode::IntcodeCpu;

fn main() {
    let input = include_str!("day2.txt");
    solve_part1(&input);
    solve_part2(&input);
}

fn solve_part1(input: &str) {
    let memory: Vec<u32> = parse_input(input);

    let res = run_with_args(&memory, 12, 2);
    println!("Part 1: {:?}", res)
}

fn solve_part2(input: &str) {
    let rom: Vec<u32> = parse_input(input);

    if let Some((n, v)) = find_matching_input(&rom, 19_690_720u32) {
        println!("Part 2: {}", 100 * n + v)
    }
}

fn find_matching_input(rom: &[u32], out: u32) -> Option<(u32, u32)> {
    all_inputs().find(|(noun, verb)| {
        run_with_args(&rom, *noun, *verb) == out
    })
}

fn all_inputs() -> impl Iterator<Item=(u32, u32)> {
    (0..100).flat_map(|noun| (0..100).map(move |verb| {
        (noun, verb)
    }))
}

fn run_with_args(rom: &[u32], noun: u32, verb: u32) -> u32 {
    let mut cpu = IntcodeCpu::new(Vec::from(rom));
    cpu.memory[1] = noun;
    cpu.memory[2] = verb;
    cpu.run();
    cpu.memory[0]
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect()
}