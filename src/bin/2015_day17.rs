use itertools::{Itertools, MultiProduct};
use std::ops::RangeInclusive;

fn main() {
    let containers = parse_containers(include_str!("../inputs/2015/day17.txt"));

    let combinations = get_combinations(&containers);

    println!("Part 1: {:?}", combinations.len());

    let min_containers = combinations.iter().min_by_key(|(c, _)| *c).unwrap().0;
    let res = combinations.iter().filter(|(c, _)| *c == min_containers).count();
    println!("Part 2: {}", res);
}

fn parse_containers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec()
}

fn total_volume(containers: &[u32], select: &[u32]) -> u32 {
    containers.iter().zip(select).map(|(a, b)| a * b).sum()
}

fn get_all_indices(n: usize) -> impl Iterator<Item=Vec<u32>> {
    (0..n).map(|_| 0u32..=1u32)
        .multi_cartesian_product()
}

fn get_combinations(containers: &Vec<u32>) -> Vec<(u32, u32)> {
    get_all_indices(containers.len())
        .map(|i| (i.iter().sum::<u32>(), total_volume(&containers, &i)))
        .filter(|(_, v)| *v == 150)
        .collect_vec()
}
