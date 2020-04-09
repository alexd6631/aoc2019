use aoc2019::vec3d::Vec3d;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::collections::HashSet;
use itertools::Itertools;
use num_integer::{gcd, gcd_lcm, lcm};

type Int = i32;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Moon {
    position: Vec3d<Int>,
    velocity: Vec3d<Int>,
}

impl Moon {
    pub fn new(x: Int, y: Int, z: Int) -> Self {
        Moon { position: Vec3d::new(x, y, z), velocity: Vec3d::new(0, 0, 0) }
    }

    pub fn energy(&self) -> i32 {
        sum_of_abs_value(&self.position) * sum_of_abs_value(&self.velocity)
    }
}

fn int_abs(n: Int) -> Int {
    if n >= 0 { n } else { -n }
}

fn sum_of_abs_value(v: &Vec3d<Int>) -> i32 {
    int_abs(v.x) + int_abs(v.y) + int_abs(v.z)
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position += self.velocity
    }
}

fn compute_gravity_vec(moon_a: &Moon, moon_b: &Moon) -> Vec3d<i32> {
    fn aux(a: Int, b: Int) -> Int {
        if a < b { 1 } else if a > b { -1 } else { 0 }
    }
    let pos_a = &moon_a.position;
    let pos_b = &moon_b.position;

    Vec3d::new(
        aux(pos_a.x, pos_b.x),
        aux(pos_a.y, pos_b.y),
        aux(pos_a.z, pos_b.z),
    )
}

fn apply_gravity(moon_a: &mut Moon, moon_b: &mut Moon) {
    let gravity_vec = compute_gravity_vec(moon_a, moon_b);

    moon_a.velocity += gravity_vec;
    moon_b.velocity -= gravity_vec;
}

fn run_step(moons: &Vec<RefCell<Moon>>) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            apply_gravity(&mut moons.get(i).unwrap().borrow_mut(),
                          &mut moons.get(j).unwrap().borrow_mut())
        }
    }
    moons.iter().for_each(|m| m.borrow_mut().apply_velocity())
}

fn solve_part_1(moons: &Vec<RefCell<Moon>>, steps: u32) -> i32 {
    for _ in 0..steps { run_step(moons) }
    moons.iter()
        .map(|m| m.borrow().energy())
        .sum()
}

fn solve_part_2(moons: &Vec<RefCell<Moon>>) -> u64 {
    let (period_x, period_y, period_z) = find_periods(moons);
    println!("{} {} {}", period_x, period_y, period_z);
    lcm(lcm(period_x, period_y), period_z)
}

fn get_x_state(moons: &Vec<RefCell<Moon>>) -> Vec<(i32, i32)> {
    moons.iter().map(|m| {
        let m = m.borrow();
        (m.position.x, m.velocity.x)
    }).collect_vec()
}

fn get_y_state(moons: &Vec<RefCell<Moon>>) -> Vec<(i32, i32)> {
    moons.iter().map(|m| {
        let m = m.borrow();
        (m.position.y, m.velocity.y)
    }).collect_vec()
}

fn get_z_state(moons: &Vec<RefCell<Moon>>) -> Vec<(i32, i32)> {
    moons.iter().map(|m| {
        let m = m.borrow();
        (m.position.z, m.velocity.z)
    }).collect_vec()
}

fn find_periods(moons: &Vec<RefCell<Moon>>) -> (u64, u64, u64) {
    let init_x_state = get_x_state(moons);
    let init_y_state = get_y_state(moons);
    let init_z_state = get_z_state(moons);

    let mut step = 0u64;
    let mut period_x: Option<u64> = None;
    let mut period_y: Option<u64> = None;
    let mut period_z: Option<u64> = None;
    loop {
        run_step(moons);
        step += 1;

        if period_x.is_none() && get_x_state(moons) == init_x_state {
            period_x = Some(step);
        }

        if period_y.is_none() && get_y_state(moons) == init_y_state {
            period_y = Some(step);
        }

        if period_z.is_none() && get_z_state(moons) == init_z_state {
            period_z = Some(step);
        }

        if let (Some(px), Some(py), Some(pz)) = (period_x, period_y, period_z) {
            return (px, py, pz)
        }
    }
}

fn main() {
    let moons = vec![
        RefCell::new(Moon::new(13, 9, 5)),
        RefCell::new(Moon::new(8, 14, -2)),
        RefCell::new(Moon::new(-5, 4, 11)),
        RefCell::new(Moon::new(2, -6, 1))
    ];

    let res = solve_part_1(&moons.clone(), 1000);
    println!("{} - {:?}", res, moons);

    let period = solve_part_2(&moons);
    println!("{}", period);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::{Moon, run_step, solve_part_1};

    #[test]
    fn test_example() {
        let moons = vec![
            RefCell::new(Moon::new(-1, 0, 2)),
            RefCell::new(Moon::new(2, -10, -7)),
            RefCell::new(Moon::new(4, -8, 8)),
            RefCell::new(Moon::new(3, 5, -1))
        ];

        let res = solve_part_1(&moons, 10);
        println!("{} - {:?}", res, moons);
    }
}