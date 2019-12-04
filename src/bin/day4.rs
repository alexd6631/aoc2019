use std::ops::Range;

fn main() {
    let input = 240920..789857;
    let res = solve_part1(input.clone());
    println!("{:?}", res);

    let res = solve_part2(input.clone());
    println!("{:?}", res)
}

fn solve_part1(input: Range<u32>) -> usize {
    input
        .map(get_digits_optimized)
        .filter(|d| always_increasing(d))
        .filter(|d| has_consecutive(d))
        .count()
}

fn solve_part2(input: Range<u32>) -> usize {
    input
        .map(get_digits_optimized)
        .filter(|d| always_increasing(d))
        .filter(|d| has_exactly_two_consecutive(d))
        .count()
}

fn get_digits_optimized(password: u32) -> [u8; 6] {
    [
        (password / 100_000) as u8,
        (password / 10_000 % 10) as u8,
        (password / 1_000 % 10) as u8,
        (password / 100 % 10) as u8,
        (password / 10 % 10) as u8,
        (password % 10) as u8,
    ]
}

fn has_consecutive(digits: &[u8]) -> bool {
    digits.windows(2).any(|sl| sl[0] == sl[1])
}

fn always_increasing(digits: &[u8]) -> bool {
    digits.windows(2).all(|sl| sl[0] <= sl[1])
}

fn has_exactly_two_consecutive(digits: &[u8]) -> bool {
    digits.iter().map(|d| {
        digits.iter().filter(|c| *c == d).count()
    }).any(|n| n == 2)
}