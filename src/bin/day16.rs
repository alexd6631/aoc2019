
use itertools::Itertools;
use std::ops::Rem;
use aoc2019::utils::char_to_i32;


struct Pattern<'a>{
    base: &'a[i32],
    repeat: usize,
    i: usize,
    j: usize
}

impl<'a> Pattern<'a> {
    pub fn new(base: &'a [i32], repeat: usize) -> Self {
        Pattern { base, repeat, i: 0, j: repeat }
    }
}

impl<'a> Iterator for Pattern<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j == 0 {
            self.j = self.repeat;
            self.i = (self.i + 1) % self.base.len()
        }
        self.j -= 1;
        Some(self.base[self.i])
    }
}

fn pattern(repeat: usize) -> impl Iterator<Item=i32> {
    Pattern::new(&[0, 1, 0, -1], repeat).skip(1)
}

fn fft(input: &[i32]) -> Vec<i32> {
    (1 ..= input.len()).map(|i| {
        let p = pattern(i);
        input.iter().zip(p)
            .map(|(a, b)| (a * b))
            .sum::<i32>().rem(10).abs()
    }).collect_vec()
}

fn fft_n(input: Vec<i32>, phases: usize) -> Vec<i32> {
    (0 .. phases).fold(input, |acc, _| {
        fft(&acc)
    })
}

fn solve_part1(input: Vec<i32>) {
    let out = fft_n(input, 100);

    println!("{}", &out[..8].iter().join(""));
}

fn solve_part2(input: Vec<i32>) {
    let input = input.repeat(100);
    let out = fft_n(input, 1);

    println!("{}", &out[..8].iter().join(""));
}

fn main() {
    let input_str = include_str!("../inputs/day16.txt");
    let input = input_str.chars()
        .map(char_to_i32)
        .collect_vec();

    solve_part1(input.clone());
    solve_part2(input);
}
