use std::collections::{BTreeMap, HashMap};

use smallvec::SmallVec;

use aoc2019::bfs::Graph;
use aoc2019::bfs_alt::bfs_alt;
use aoc2019::intcode::{Int, IntcodeCpu, parse_intcode_program};

struct World {
    blocks: BTreeMap<(i32, i32), Block>
}

impl World {
    pub fn new() -> Self {
        let mut blocks = BTreeMap::new();
        blocks.insert((0, 0), Block::Path);
        World { blocks }
    }
}

#[derive(Copy, Clone, Debug)]
enum Block {
    Path,
    Wall,
    Goal,
}

impl Block {
    fn is_path(self) -> bool {
        match self {
            Block::Path => true,
            _ => false
        }
    }

    fn is_goal(self) -> bool {
        match self {
            Block::Goal => true,
            _ => false
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Move {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Graph for World {
    type Node = (i32, i32);
    type Edge = Move;
    type Adjacents = SmallVec<[(Self::Edge, Self::Node); 4]>;

    fn adjacents(&self, node: &Self::Node) -> Self::Adjacents {
        if let Some(block) = self.blocks.get(node) {
            match block {
                Block::Wall => {
                    SmallVec::new()
                }
                _ => {
                    [Move::North, Move::South, Move::West, Move::East]
                        .iter()
                        .map(|m| (*m, apply_move(*node, *m)))
                        .collect()
                }
            }
        } else {
            SmallVec::new()
        }
    }
}

fn apply_move(pos: (i32, i32), m: Move) -> (i32, i32) {
    let (x, y) = pos;
    match m {
        Move::North => (x, y + 1),
        Move::South => (x, y - 1),
        Move::West => (x - 1, y),
        Move::East => (x + 1, y),
    }
}

struct Drone {
    cpu: IntcodeCpu,
    pos: (i32, i32)
}

impl Drone {
    pub fn new() -> Self {
        let source = include_str!("../inputs/day15.txt");
        let program = parse_intcode_program(source);
        let cpu = IntcodeCpu::new_with_inputs_and_large_mem(64_000, program, Vec::new());
        Drone { cpu, pos: (0, 0) }
    }

    fn try_move(&mut self, m: Move) -> (Block, (i32, i32)) {
        self.cpu.inputs.push_back(m as Int);
        self.cpu.run();
        let new_pos = apply_move(self.pos, m);
        let new_block = match self.cpu.outputs.pop().unwrap() {
            0 => Block::Wall,
            1 => {
                self.pos = new_pos;
                Block::Path
            }
            2 => {
                self.pos = new_pos;
                Block::Goal
            }
            _ => panic!("Unreachable")
        };
        (new_block, new_pos)
    }
}

fn solve_part_1() {
    let mut world = World::new();
    let mut drone = Drone::new();

    loop {
        let path_to_unknown = bfs_alt(&world, drone.pos, |g, n| {
            g.blocks.get(n).is_none()
        }).unwrap();

        for m in path_to_unknown {
            let (new_block, new_pos) = drone.try_move(m);
            world.blocks.insert(new_pos, new_block);
            if new_block.is_goal() {
                print_path_to_objective(&world, drone.pos);
                return;
            }
        }
    }
}

fn solve_part_2() {
    let mut drone = Drone::new();
    let mut world = World::new();

    let goal = explore_all_map(&mut drone, &mut world);
    println!("Done exploring, goal = {:?}", goal);

    let depth = world.blocks.iter()
        .filter(|(_, block)| block.is_path())
        .map(|(pos, _)| bfs_alt(&world, *pos, |_, n| n == &goal).unwrap().len())
        .max()
        .unwrap();

    println!("Max depth: {:?}", depth);
}

fn explore_all_map(drone: &mut Drone, world: &mut World) -> (i32, i32) {
    let mut goal = (0, 0);

    while let Some(path_to_unknown) = bfs_alt(world, drone.pos, |g, n| {
        g.blocks.get(n).is_none()
    }) {
        for m in path_to_unknown {
            let (new_block, new_pos) = drone.try_move(m);
            world.blocks.insert(new_pos, new_block);
            if new_block.is_goal() {
                goal = new_pos;
            }
        }
    }
    goal
}

fn print_path_to_objective(world: &World, goal: (i32, i32)) {
    println!("Found objective ! {:?}", goal);
    let path = bfs_alt(world, (0, 0), |_, n| {
        n == &goal
    });
    println!("Result : {:?}", path.unwrap().len());
}

fn main() {
    solve_part_1();
    solve_part_2()
}