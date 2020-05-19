use std::fmt::{Display, Error, Formatter};
use std::mem::swap;
use std::ptr::replace;
use std::thread::sleep;
use std::time::Duration;

use itertools::Itertools;

use aoc2019::intcode::{Int, IntcodeCpu, parse_intcode_program};

fn part1(input: &str) -> usize {
    let mut cpu = IntcodeCpu::new_with_inputs_and_large_mem(64_000, parse_intcode_program(input), vec![]);
    cpu.run();
    cpu.outputs.chunks(3)
        .filter(|v| v[2] == 2)
        .count()
}

struct Game {
    cpu: IntcodeCpu,
    buffer: [[Int; 38]; 21],
    paddle: (Int, Int),
    ball: (Int, Int),
    score: Int,
}

impl Game {
    pub fn new(mut cpu: IntcodeCpu) -> Self {
        cpu.memory[0] = 2;
        Game {
            cpu,
            buffer: [[0; 38]; 21],
            paddle: (0, 0),
            ball: (0, 0),
            score: 0,
        }
    }

    fn start(&mut self) {
        self.cpu.run();
        self.process_outputs();
    }

    fn process_outputs(&mut self) {
        let outputs = std::mem::replace(&mut self.cpu.outputs, vec![]);
        if let Some(player_pos) = outputs.chunks(3).find(|c| c[0] != -1 && c[2] == 3) {
            self.paddle = (player_pos[0], player_pos[1]);
        }
        if let Some(ball_pos) = outputs.chunks(3).find(|c| c[0] != -1 && c[2] == 4) {
            self.ball = (ball_pos[0], ball_pos[1]);
        }

        if let Some(led) = outputs.chunks(3).find(|c| c[0] == -1) {
            self.score = led[2];
        }

        outputs.chunks(3).for_each(|c| {
            if c[0] != -1 {
                self.buffer[c[1] as usize][c[0] as usize] = c[2];
            }
        });
    }

    fn play(&mut self, direction: Int) {
        self.cpu.inputs.push_back(direction);
        self.start();
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Score: {}", self.score)?;
        writeln!(f)?;
        for i in 0..21 {
            for j in 0..38 {
                write!(f, "{}", char_for_code(self.buffer[i][j]))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn char_for_code(c: Int) -> char {
    match c {
        1 => '#',
        2 => '@',
        3 => '=',
        4 => 'o',
        _ => ' '
    }
}


fn part2(input: &str) {
    let program = parse_intcode_program(input);
    let cpu = IntcodeCpu::new_with_inputs_and_large_mem(64_000, program, vec![]);
    let mut game = Game::new(cpu);
    game.start();

    while !game.cpu.is_halted {
        let diff = game.paddle.0 - game.ball.0;
        let next_move = if diff == 0 { 0 } else if diff > 0 { -1 } else { 1 };
        game.play(next_move);

        // print!("\x1B[2J");
        // println!("{}", game);
        // sleep(Duration::from_millis(16));
    }
    println!("{}", game);
}


fn main() {
    let input = include_str!("../inputs/day13.txt");

    let res = part1(input);
    println!("Part 1: {}", res);
    part2(input);
}