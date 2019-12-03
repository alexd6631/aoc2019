use std::collections::BTreeMap;

fn main() {
    let input = include_str!("day3.txt");

    let world = run_wires(input);

    let dist = solve_part1(&world);
    println!("{:?}", dist);

    let steps = solve_part2(&world);
    println!("{:?}", steps);
}

fn solve_part1(world: &World) -> i32 {
    let mut inter_distances: Vec<_> = world.intersections()
        .map(|(pos, _)| pos)
        .map(|(x, y)| i32::abs(*x) + i32::abs(*y))
        .collect();

    inter_distances.sort();
    inter_distances[0]
}

fn solve_part2(world: &World) -> u32 {
    let mut inter_steps: Vec<_> = world.intersections()
        .map(|(_, v)| v[0].1 + v[1].1)
        .collect();

    inter_steps.sort();
    inter_steps[0]
}

fn run_wires(input: &str) -> World {
    let (a_moves, b_moves) = parse_all_moves(input);
    let mut world = World::default();
    let mut wire_a = Wire::new(Id(0));
    let mut wire_b = Wire::new(Id(1));
    a_moves.iter().for_each(|m| wire_a.apply_move(&mut world, &m));
    b_moves.iter().for_each(|m| wire_b.apply_move(&mut world, &m));
    world
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Id(u8);

type Pos = (i32, i32);
type MoveInt = usize;
type Step = u32;


#[derive(Debug, Default)]
struct World {
    map: BTreeMap<Pos, Vec<(Id, Step)>>
}

impl World {
    fn mark(&mut self, wire: &Wire) {
        self.map.entry(wire.pos)
            .and_modify(|v| if !v.iter().any(|(id, _)| id == &wire.id) {
                v.push((wire.id, wire.step))
            })
            .or_insert_with(|| vec![(wire.id, wire.step)]);
    }

    fn intersections(&self) -> impl Iterator<Item=(&Pos, &Vec<(Id, Step)>)> {
        self.map.iter()
            .filter(|(_, v)| v.len() == 2)
    }
}

struct Wire {
    id: Id,
    pos: Pos,
    step: Step
}

impl Wire {
    pub fn new(id: Id) -> Self {
        Wire { id, pos: (0, 0), step: 0 }
    }
}

#[derive(Debug)]
enum Move {
    Left(MoveInt),
    Right(MoveInt),
    Up(MoveInt),
    Down(MoveInt),
}

impl Wire {

    fn apply_move(&mut self, world: &mut World, a_move: &Move) {
        macro_rules! move_and_mark {
            ($n: expr, $m:expr)  => {
                for _ in 0 .. *$n {
                    $m;
                    self.step += 1;
                    world.mark(&self);
                }
            };
        }

        match a_move {
            Move::Left(n) =>
                move_and_mark! {n, self.pos.0 -= 1 },
            Move::Right(n) =>
                move_and_mark! {n, self.pos.0 += 1 },
            Move::Up(n) =>
                move_and_mark! {n, self.pos.1 += 1 },
            Move::Down(n) =>
                move_and_mark! {n, self.pos.1 -= 1 }
        }
    }
}

fn parse_all_moves(input: &str) -> (Vec<Move>, Vec<Move>) {
    let mut s = input.split('\n');
    let first_moves = parse_moves(s.next().unwrap());
    let second_moves = parse_moves(s.next().unwrap());
    (first_moves, second_moves)
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.split(',').map(|m| {
        let dir = &m.chars().nth(0).unwrap();
        let n: MoveInt = m[1..].parse().unwrap();

        match dir {
            'U' => Move::Up(n),
            'D' => Move::Down(n),
            'L' => Move::Left(n),
            'R' => Move::Right(n),
            _ => panic!("Invalid move")
        }
    }).collect()
}