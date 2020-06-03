use aoc2019::intcode::{IntcodeCpu, Int, run_with_inputs, parse_intcode_program};
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let prog = parse_intcode_program(input);
    let res = max_signal_for_settings(&prog);

    println!("Part 1: {}", res);

    let res = max_signal_for_settings_part_2(&prog);
    println!("Part 2: {}", res);
}

fn max_signal_for_settings(program: &Vec<Int>) -> Int {
    [0i64, 1, 2, 3, 4].iter().permutations(5).map(|p| {
        let vec: Vec<Int> = p.iter().map(|n| **n).collect();
        signal_for_settings(&(vec), program)
    }).max().unwrap()
}

fn signal_for_settings(phase_settings: &[Int], program: &Vec<Int>) -> Int {
    phase_settings.iter().fold(0 as Int, |input, setting| {
        *run_with_inputs(program.clone(), vec![*setting, input]).first().expect("No output")
    })
}

fn max_signal_for_settings_part_2(program: &Vec<Int>) -> Int {
    [5i64, 6, 7, 8, 9].iter().permutations(5).map(|p| {
        let vec: Vec<Int> = p.iter().map(|n| **n).collect();
        signal_for_settings_part2(&(vec), program)
    }).max().unwrap()
}


fn signal_for_settings_part2(phase_settings: &[Int], program: &Vec<Int>) -> Int {
    let mut cpus: Vec<_> = phase_settings.iter()
        .map(|setting| IntcodeCpu::new_with_inputs(program.clone(), vec![*setting]))
        .collect();

    cpus[0].inputs.push_back(0);

    let n = phase_settings.len();
    loop {
        for i in 0..n {
            cpus[i].run();
            let next_index = (i + 1) % n;

            if cpus[next_index].is_halted {
                return cpus[i].outputs[0];
            } else {
                let out = cpus[i].outputs.pop().unwrap();
                cpus[next_index].inputs.push_back(out);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use aoc2019::intcode::parse_intcode_program;
    use crate::{signal_for_settings, signal_for_settings_part2, max_signal_for_settings_part_2};

    #[test]
    fn test_signal_for_settings() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let prog = parse_intcode_program(input);
        let res = signal_for_settings(&vec![4, 3, 2, 1, 0], &prog);

        assert_eq!(res, 43210);
    }

    #[test]
    fn test_signal_for_settings_2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let prog = parse_intcode_program(input);
        let res = signal_for_settings(&vec![0, 1, 2, 3, 4], &prog);

        assert_eq!(res, 54321);
    }

    #[test]
    fn test_signal_for_settings_part_2() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let prog = parse_intcode_program(input);
        let res = signal_for_settings_part2(&vec![9, 8, 7, 6, 5], &prog);

        assert_eq!(res, 139629729);
    }

    #[test]
    fn test_max_signal_for_settings_part_2() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let prog = parse_intcode_program(input);
        let res = max_signal_for_settings_part_2(&prog);

        assert_eq!(res, 139629729);
    }
}