use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let tape = include_str!("../inputs/2015/day16/tape.txt");
    let inventory: HashMap<String, u32> = tape.lines().map(|l| {
        let parts: Vec<&str> = l.split(":").collect();
        (parts[0].to_owned(), parts[1].trim().parse::<u32>().unwrap())
    }).collect();

    println!("{:?}", inventory);

    let inventory_set = to_set(&inventory);
    let input = include_str!("../inputs/2015/day16/input.txt");

    let sues = input.lines().map(|l| {
        let parts: Vec<&str> = l.splitn(2, ":").collect();
        let n = parts[0].split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

        let inv: HashMap<String, u32> = parts[1].split(",").map(|p| {
            let parts = p.split(":").collect::<Vec<&str>>();
            (parts[0].trim().to_owned(), parts[1].trim().parse::<u32>().unwrap())
        }).collect();
        (n, inv)
    }).collect_vec();


    let res = sues.iter()
        .find(|(_, inv)| to_set(&inv).is_subset(&inventory_set));
    println!("{:?}", res);

    let res = sues.iter()
        .find(|(_, inv)| check_aunt(&inventory, inv));
    println!("{:?}", res);
}

fn to_set(inventory: &HashMap<String, u32>) -> HashSet<(&str, u32)> {
    inventory.iter()
        .map(|(s, i)| (s.as_str(), *i))
        .collect()
}

fn check_aunt(my: &HashMap<String, u32>, sue: &HashMap<String, u32>) -> bool {

    sue.iter().all(|(prop, val)| {

        match prop.as_str() {
            "cats" | "trees" => *val > my[prop],
            "pomeranians" | "goldfish" => *val < my[prop],
            _ => *val == my[prop]
        }
    })
}