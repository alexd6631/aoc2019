use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Asteroid {
    x: u16,
    y: u16,
}


#[derive(Debug)]
struct Target<'a> {
    asteroid: &'a Asteroid,
    angle: f64,
    distance: f64,
    depth: usize,
}

impl Asteroid {
    pub fn new(x: u16, y: u16) -> Self {
        Asteroid { x, y }
    }

    fn n_visibles(&self, others: &[Asteroid]) -> usize {
        others
            .iter()
            .filter(|&o| o != self)
            .map(|a| {
                let dx = a.x as f64 - self.x as f64;
                let dy = a.y as f64 - self.y as f64;
                dy.atan2(dx)
            })
            .map(|i| (i * 1e15) as i64)
            .unique().count()
    }

    fn target<'a>(&self, other: &'a Asteroid) -> Target<'a> {
        let dx = other.x as f64 - self.x as f64;
        let dy = other.y as f64 - self.y as f64;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = (-dx).atan2(dy);

        Target {
            asteroid: other,
            distance,
            angle,
            depth: 0,
        }
    }
}

fn parse_asteroids(input: &str) -> Vec<Asteroid> {
    input.lines()
        .enumerate()
        .flat_map(|(row, line)|
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Asteroid::new(col as u16, row as u16))
        ).collect_vec()
}

fn solve_part_1(asteroids: &[Asteroid]) -> (&Asteroid, usize) {
    asteroids.iter()
        .map(|a| (a, a.n_visibles(asteroids)))
        .max_by_key(|(_, n)| *n)
        .unwrap()
}

fn solve_part_2(best_asteroid: &Asteroid, asteroids: &[Asteroid]) -> u16 {
    let mut targets = build_targets(best_asteroid, asteroids);

    compute_depths(&mut targets);

    targets.sort_by(|a, b| {
        a.depth.cmp(&b.depth).then(a.angle.partial_cmp(&b.angle).unwrap())
    });

    let found = targets[199].asteroid;
    found.x * 100 + found.y
}

fn build_targets<'a>(best_asteroid: &Asteroid, asteroids: &'a [Asteroid]) -> Vec<Target<'a>> {
    asteroids
        .iter()
        .filter(|&o| o != best_asteroid)
        .map(|a| best_asteroid.target(a))
        .sorted_by(|a, b|
            a.angle.partial_cmp(&b.angle).unwrap()
                .then(a.distance.partial_cmp(&b.distance).unwrap())
        ).collect_vec()
}

fn compute_depths(targets: &mut Vec<Target>) {
    for i in 0..targets.len() - 1 {
        let a = &targets[i];
        let angle = a.angle;
        let depth = a.depth;
        let b = &mut targets[i + 1];
        if b.angle == angle {
            b.depth = depth + 1
        }
    }
}

fn main() {
    let asteroids = parse_asteroids(include_str!("../inputs/day10.txt"));
    let (best_asteroid, n_visible) = solve_part_1(&asteroids);
    println!("{} visible asteroids from: ({}, {})", n_visible, best_asteroid.x, best_asteroid.y);

    let res = solve_part_2(best_asteroid, &asteroids);
    println!("{}", res);

}

#[cfg(test)]
mod test {
    use crate::{parse_asteroids, solve_part_1, Asteroid, solve_part_2};

    #[test]
    fn test_parse() {
        let asteroids = parse_asteroids(include_str!("../inputs/examples/day10_example1.txt"));
        println!("{:?}", asteroids)
    }

    #[test]
    fn test_n_visible() {
        let asteroids = parse_asteroids(include_str!("../inputs/examples/day10_example1.txt"));

        assert_eq!(7, asteroids[0].n_visibles(&asteroids));
        assert_eq!(7, asteroids[1].n_visibles(&asteroids));
        assert_eq!(6, asteroids[2].n_visibles(&asteroids));
    }

    #[test]
    fn test_solver_part1() {
        let asteroids = parse_asteroids(include_str!("../inputs/examples/day10_example1.txt"));

        let (_, res) = solve_part_1(&asteroids);
        assert_eq!(8, res);
    }

    #[test]
    fn test_solver_part1_large() {
        let asteroids = parse_asteroids(include_str!("../inputs/examples/day10_example_large.txt"));

        let (_, res) = solve_part_1(&asteroids);
        assert_eq!(210, res);
    }

    #[test]
    fn test_target() {
        let a = Asteroid::new(4, 4);

        println!("{:?}", a.target(&Asteroid::new(8, 0)));
        println!("{:?}", a.target(&Asteroid::new(4, 0)));
        println!("{:?}", a.target(&Asteroid::new(0, 0)));
        println!("{:?}", a.target(&Asteroid::new(4, 8)));
    }
    #[test]
    fn test_part_2() {
        let asteroids = parse_asteroids(include_str!("../inputs/examples/day10_example_large.txt"));

        let (b, _) = solve_part_1(&asteroids);
        let res = solve_part_2(b, &asteroids);
        assert_eq!(802, res);
    }
}