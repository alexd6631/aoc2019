use std::collections::{HashMap, HashSet};
use aoc2019::bfs::Graph;
use aoc2019::bfs_alt::bfs_alt;
use itertools::Itertools;

fn parse_replacements(input: &str) -> Replacements {
    Replacements(input.lines()
        .map(|l| {
            let parts: Vec<&str> = l.split("=>").collect();
            (parts[0].trim().to_owned(), parts[1].trim().to_owned())
        }).collect())
}

#[derive(Debug)]
struct Replacements(Vec<(String, String)>);

impl Replacements {
    fn get_all_possible_replacements<'a>(&'a self, input: &'a str) -> impl Iterator<Item=String> + 'a {
        self.0.iter().flat_map(move |(p, r)| {
            input.match_indices(p)
                .map(move |(i, _)| {
                    let mut new = input.to_owned();
                    new.replace_range(i..i + p.len(), r);
                    new
                })
        })
    }

    fn get_all_reverse_replacements<'a>(&'a self, input: &'a str) -> impl Iterator<Item=String> + 'a {
        self.0.iter().flat_map(move |(r, p)| {
            input.match_indices(p)
                .map(move |(i, _)| {
                    let mut new = input.to_owned();
                    new.replace_range(i..i + p.len(), r);
                    new
                })
        })
    }

    fn get_all_possible_replacements_set(&self, input: &str) -> HashSet<String> {
        self.get_all_possible_replacements(input).collect()
    }
}

fn main() {
    let replacements = parse_replacements(include_str!("../inputs/2015/day19/replacements.txt"));
    let medicine = include_str!("../inputs/2015/day19/medicine.txt");

    let all_replacements = replacements.get_all_possible_replacements_set(medicine);
    println!("{}", all_replacements.len());

    let search_graph = SearchGraph::new(&replacements);
    let path = bfs_alt(&search_graph, medicine.to_string(), |_, m| {
        m == "e"
    });
    if let Some(path) = path {
        println!("{:?}", path.len());
    } else {
        println!("Not found");
    }
}

struct SearchGraph<'a> {
    replacement: &'a Replacements,
}

impl<'a> SearchGraph<'a> {
    pub fn new(replacement: &'a Replacements) -> Self {
        SearchGraph { replacement }
    }
}

impl<'a> Graph for SearchGraph<'a> {
    type Node = String;
    type Edge = ();
    type Adjacents = Vec<((), String)>;

    fn adjacents(&self, node: &Self::Node) -> Self::Adjacents {
        let subs = self.replacement.get_all_reverse_replacements(node.as_str())
            .collect_vec();

        if subs.is_empty() {
            vec![]
        } else {
            let min = subs.iter().map(|s| s.len()).min().unwrap();
            subs.into_iter()
                .filter(|s| s.len() == min)
                .map(|s| ((), s))
                .take(2)
                .collect()
        }

    }


}