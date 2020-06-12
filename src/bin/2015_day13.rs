use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Rules(HashMap<(u8, u8), i32>);

impl Rules {
    fn add_rule(&mut self, from: u8, to: u8, gain: i32) {
        self.0.insert((from, to), gain);
    }
}

fn parse_rules(input: &str) -> (Rules, HashMap<String, u8>) {
    let name_to_index: HashMap<String, u8> = input.lines()
        .map(|l| l.split_ascii_whitespace().collect_vec()[0])
        .unique()
        .enumerate()
        .map(|(a, b)| (b.to_string(), a as u8))
        .inspect(|name| println!("{:?}", name))
        .collect();

    println!("{:?}", name_to_index);
    let mut rules = Rules::default();
    input
        .replace(".", "")
        .lines()
        .map(|l| parse_rule(&name_to_index, l))
        .for_each(|(from, to, gain)| rules.add_rule(from, to, gain));
    (rules, name_to_index)
}

fn parse_rule(mapping: &HashMap<String, u8>, line: &str) -> (u8, u8, i32) {
    let tokens = line.split_ascii_whitespace().collect_vec();
    let from = tokens[0];
    let from = mapping.get(from).unwrap();

    let gain_abs: i32 = tokens[3].parse().unwrap();
    let gain = if tokens[2] == "lose" { -gain_abs } else { gain_abs };

    let to = tokens[tokens.len() - 1];
    let to = mapping.get(to).unwrap();

    (*from, *to, gain)
}

fn compute_hapiness(rules: &Rules, permutation: &[u8]) -> i32 {
    let n = permutation.len();
    permutation.iter()
        .enumerate()
        .map(|(i, from)|
            *rules.0.get(&(*from, permutation[(i + n - 1) % n])).unwrap()
            + *rules.0.get(&(*from, permutation[(i + 1) % n])).unwrap()
        )
        .sum()
}

fn solve_part_1(input: &str) -> i32 {
    let (rules, name_to_index) = parse_rules(input);

    let n_persons = name_to_index.len();
    let init = (0u8 ..n_persons as u8).collect_vec();

    init.iter().permutations(n_persons)
        .map(|p| p.iter().map(|c| **c).collect_vec())
        .map(|p| compute_hapiness(&rules, &p))
        .max().unwrap()
}

fn solve_part_2(input: &str) -> i32 {
    let (mut rules, name_to_index) = parse_rules(input);
    let n_persons = name_to_index.len() as u8;
    let me = n_persons;
    (0u8 .. n_persons)
        .for_each(|p| {
            rules.add_rule(me, p, 0);
            rules.add_rule(p, me, 0);
        });

    let init = (0u8 .. (n_persons + 1)).collect_vec();

    init.iter().permutations((n_persons + 1) as usize)
        .map(|p| p.iter().map(|c| **c).collect_vec())
        .map(|p| compute_hapiness(&rules, &p))
        .max().unwrap()
}

fn main() {
    let input = include_str!("../inputs/2015/day13.txt");

    let max1 = solve_part_1(&input);
    let max2 = solve_part_2(&input);

    println!("{} {}", max1, max2);
}

const SAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";