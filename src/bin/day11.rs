use aoc2019::intcode::{IntcodeCpu, Int, parse_intcode_program};
use crate::Color::Black;
use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct DrawingProgram {
    cpu: IntcodeCpu
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl Color {
    fn parse(v: u8) -> Option<Color> {
        match v {
            0 => Some(Color::Black),
            1 => Some(Color::White),
            _ => None
        }
    }
}

enum Turn {
    Left, Right
}

impl Turn {
    fn parse(v: u8) -> Option<Turn> {
        match v {
            0 => Some(Turn::Left),
            1 => Some(Turn::Right),
            _ => None
        }
    }
}

enum Direction {
    Left, Up, Right, Down
}

impl Direction {
    fn apply(&self, turn: &Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
            },
            Turn::Right => match self {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            },
        }
    }
}

impl DrawingProgram {
    fn next(&mut self, current: Color) -> Option<(Color, Turn)> {
        self.cpu.inputs.push_back(current as Int);
        self.cpu.run();
        if self.cpu.outputs.len() == 2 {
            let color = Color::parse(self.cpu.outputs[0] as u8).unwrap();
            let turn = Turn::parse(self.cpu.outputs[1] as u8).unwrap();
            self.cpu.outputs.clear();
            Some((color, turn))
        } else if self.cpu.is_halted {
            None
        } else {
            panic!("illegal state")
        }
    }
}

struct Robot {
    x: i32,
    y: i32,
    direction: Direction
}

impl Robot {
    fn turn_and_forward(&mut self, turn: &Turn) {
        self.direction = self.direction.apply(turn);
        match self.direction {
            Direction::Left => {
                self.x -= 1
            },
            Direction::Up => {
                self.y += 1
            },
            Direction::Right => {
                self.x += 1
            },
            Direction::Down => {
                self.y -= 1
            },
        }
    }
}

struct Floor {
    panels: HashMap<(i32, i32), Color>
}

impl Floor {
    pub fn new() -> Self {
        Floor { panels: HashMap::new() }
    }

    pub fn new_initial(initial: Color) -> Self {
        let mut panels = HashMap::new();
        panels.insert((0, 0), initial);
        Floor { panels }
    }

    pub fn colorized_panels(&self) -> usize {
        self.panels.len()
    }
}

impl Floor {
    fn colorize(&mut self, x: i32, y: i32, color: Color) {
        self.panels.insert((x, y), color);
    }

    fn color_at(&self, x: i32, y: i32) -> Color {
        *self.panels.get(&(x, y)).unwrap_or(&Color::Black)
    }

    fn print(&self) {
        let min_x = self.panels.keys().map(|k| k.0).min().unwrap();
        let max_x = self.panels.keys().map(|k| k.0).max().unwrap();
        let min_y = self.panels.keys().map(|k| k.1).min().unwrap();
        let max_y = self.panels.keys().map(|k| k.1).max().unwrap();

        for y in (min_y ..= max_y).rev() {
            for x in min_x ..= max_x {
                if self.color_at(x,y) == Color::White {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

fn solve_part_1(mut drawing_program: DrawingProgram) -> usize {
    let mut floor = Floor::new();
    let mut robot = Robot { x: 0, y: 0, direction: Direction::Up };

    run_robot(&mut drawing_program, &mut floor, &mut robot);
    floor.colorized_panels()
}

fn solve_part_2(mut drawing_program: DrawingProgram) {
    let mut floor = Floor::new_initial(Color::White);
    let mut robot = Robot { x: 0, y: 0, direction: Direction::Up };

    run_robot(&mut drawing_program, &mut floor, &mut robot);

    floor.print();

}

fn run_robot(
    drawing_program: &mut DrawingProgram,
    floor: &mut Floor,
    robot: &mut Robot
) {
    loop {
        let current = floor.color_at(robot.x, robot.y);
        if let Some((color, turn)) = drawing_program.next(current) {
            floor.colorize(robot.x, robot.y, color);
            robot.turn_and_forward(&turn)
        } else {
            break
        }
    }
}

fn main() {
    let drawing_program = build_drawing_program();
    let res = solve_part_1(drawing_program.clone());
    println!("{}", res);
    solve_part_2(drawing_program);
}

fn build_drawing_program() -> DrawingProgram {
    let program_src = include_str!("../inputs/day11.txt");
    let program = parse_intcode_program(program_src);
    let cpu = IntcodeCpu::new_with_inputs_and_large_mem(64 * 1024, program, vec![]);
    let drawing_program = DrawingProgram { cpu };
    drawing_program
}