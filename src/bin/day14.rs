use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::RandomState;
use std::cmp::min;
use std::pin::Pin;
use std::borrow::Borrow;

#[derive(Debug)]
struct ProductionRule {
    produced: u64,
    required: Vec<(u64, String)>,
}

impl ProductionRule {
    pub fn new(produced: u64, required: Vec<(u64, String)>) -> Self {
        ProductionRule { produced, required }
    }
}

type Rules = HashMap<String, ProductionRule>;

fn parse_elem(input: &str) -> (u64, String) {
    scan_fmt::scan_fmt!(input, "{d} {}", u64, String).unwrap()
}

fn parse(input: &str) -> Rules {
    input.lines().map(|line| {
        let parts = line.split("=>").collect::<Vec<_>>();
        let left: &str = parts[0];
        let right = parts[1];
        let (n_produced, elem_produced) = parse_elem(right);
        let required = left.split(",").map(parse_elem).collect::<Vec<_>>();

        (elem_produced, ProductionRule::new((n_produced), required))
    }).collect()
}

#[derive(Debug)]
struct Solver<'a> {
    rules: &'a Rules,
    required: VecDeque<(u64, &'a str)>,
    remaining_map: HashMap<&'a str, u64>,
    ore: u64,
}

impl<'a> Solver<'a> {
    pub fn new(rules: &'a Rules) -> Self {
        Solver {
            rules,
            required: VecDeque::new(),
            remaining_map: HashMap::new(),
            ore: 0,
        }
    }

    pub fn require_fuel(&mut self, count: u64) {
        self.required.push_back((count, "FUEL"));
    }

    pub fn solve(&mut self) -> u64 {
        while !self.required.is_empty() {
            let (goal_qty, goal_elem) = self.required.pop_front().unwrap();
            if goal_elem == "ORE" {
                self.ore += goal_qty;
            } else {
                let mut remaining = *self.remaining_map.get(goal_elem).unwrap_or(&0);
                if remaining > 0 {
                    let to_use = min(remaining, goal_qty);
                    remaining -= to_use;
                    self.remaining_map.insert(goal_elem, remaining);
                    self.generate_goal(goal_qty - to_use, goal_elem);
                } else {
                    self.generate_goal(goal_qty, &goal_elem)
                }
            }
        }
        self.ore
    }

    fn generate_goal(&mut self, goal_qty: u64, goal_elem: &'a str) {
        let rule = &self.rules[goal_elem];
        let mut times = goal_qty / rule.produced;
        let has_remaining = goal_qty % rule.produced > 0;
        if has_remaining {
            times += 1;
        }
        for (n, e) in &rule.required {
            self.required.push_back((n * times, e))
        }
        if has_remaining {
            let remaining = rule.produced * times - goal_qty;
            *self.remaining_map.entry(goal_elem).or_insert(0) += remaining;
        }
    }
}

fn solve_part_1(rules: &HashMap<String, ProductionRule>) -> u64 {
    let mut solver = Solver::new(rules);
    solver.require_fuel(1);
    solver.solve()
}

fn solve_part_2(rules: &HashMap<String, ProductionRule>) -> u64 {
    let mut fuel = 1760742;
    let mut solver = Solver::new(rules);
    solver.require_fuel(fuel);
    loop {
        let ore = solver.solve();
        if ore <= 1000000000000 {
            fuel += 1;
            solver.require_fuel(1);
        } else {
            println!("{:?}", solver);
            break;
        }
    }
    fuel - 1
}


fn main() {
    let rules = parse(include_str!("../inputs/day14.txt"));
    println!("{:?}", rules);

    println!("{:?}", solve_part_1(&rules));
    println!("{:?}", solve_part_2(&rules));
}