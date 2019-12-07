use aoc2019::bfs::{Graph, bfs};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day6.txt");
    let res = solve_part1(input);
    println!("Part 1 : {}", res);

    let res = solve_part2(input);
    println!("Part 2 : {}", res);
}

fn orbit_graph_from_transitions(transitions: Vec<(String, String)>) -> OrbitGraph {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in transitions {
        orbits.entry(a).or_insert(vec![]).push(b);
    }
    OrbitGraph::new(String::from("COM"), orbits)
}

fn parse_transitions(input: &str) -> Vec<(String, String)> {
    input.lines().map(|l| {
        let i = l.find(')').expect("line not containing )");
        let (from, to) = l.split_at(i);
        (from.to_owned(), to[1..].to_owned())
    }).collect()
}

fn solve_part1(input: &str) -> usize {
    let transitions = parse_transitions(input);
    let graph = orbit_graph_from_transitions(transitions);
    let (explorer, _) = bfs(&&graph, &graph.initial, |_| false);
    explorer.cache.iter().fold(0, |acc, n| acc + n.0.depth)
}


fn solve_part2(input: &str) -> usize {
    let transitions = add_reversed_transitions(parse_transitions(input));
    let mut graph = orbit_graph_from_transitions(transitions);
    graph.initial = String::from("YOU");
    let (_, path) = bfs(&&graph, &graph.initial, |n| *n == "SAN");

    path.expect("A path was not found").len() - 2
}

fn add_reversed_transitions(transitions: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut transitions = transitions;
    let mut reversed = transitions.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
    transitions.append(&mut reversed);
    transitions
}

#[derive(Debug)]
struct OrbitGraph {
    initial: String,
    orbits: HashMap<String, Vec<String>>
}

impl OrbitGraph {
    pub fn new(initial: String, orbits: HashMap<String, Vec<String>>) -> Self {
        OrbitGraph { initial, orbits }
    }
}

impl<'a> Graph for &'a OrbitGraph {
    type Node = &'a str;
    type Edge = ();
    type Adjacents = Vec<(Self::Edge, Self::Node)>;

    fn adjacents(&self, node: &Self::Node) -> Self::Adjacents {
        self.orbits.get(*node).map(|adj| adj.iter().map(|e| ((), e.as_str()))
            .collect())
            .unwrap_or(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_part_1() {
        let res = solve_part1("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");
        assert_eq!(res, 42);
    }

    #[test]
    fn test_part_2() {
        let res = solve_part2("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN");
        assert_eq!(res, 4);
    }
}