use itertools::Itertools;
use std::fmt::Display;
use smallvec::alloc::fmt::Formatter;
use std::fmt::Write;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn is_alive(self) -> bool {
        match self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
    back_buffer: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for dc in -1i32..=1 {
            for dr in -1i32..=1 {
                if !(dc == 0 && dr == 0) {
                    let c = column as i32 + dc;
                    let r = row as i32 + dr;

                    if c >= 0 && c < (self.width as i32) && r >= 0 && r < (self.height as i32) {
                        let idx = self.get_index(r as u32, c as u32);
                        count += self.cells[idx] as u8;
                    }
                }
            }
        }

        count
    }

    pub fn tick(&mut self) {
        self.compute_next();
        std::mem::swap(&mut self.cells,
                       &mut self.back_buffer);
    }


    fn stuck_corner_lights(&mut self) {
        let a = self.get_index(0, 0);
        let b = self.get_index(self.width - 1, 0);
        let c = self.get_index(0, self.height - 1);
        let d = self.get_index(self.width - 1, self.height - 1);
        self.cells[a] = Cell::Alive;
        self.cells[b] = Cell::Alive;
        self.cells[c] = Cell::Alive;
        self.cells[d] = Cell::Alive;
    }

    pub fn tick_stuck(&mut self) {
        self.tick();
        self.stuck_corner_lights();
    }

    fn compute_next(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.compute_next_cell(row, col);
            }
        }
    }

    fn compute_next_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let neighbors = self.live_neighbor_count(row, col);

        let next_cell = match cell {
            Cell::Alive =>  match neighbors {
                2 | 3 => Cell::Alive,
                _ => Cell::Dead
            }
            Cell::Dead => match neighbors {
                3 => Cell::Alive,
                _ => Cell::Dead
            }
        };

        self.back_buffer[idx] = next_cell;
    }

    pub fn count_alive(&self) -> usize {
        self.cells.iter().filter(|c| c.is_alive()).count()
    }

    pub fn new(width: u32, height: u32, input: &str) -> Self {
        let cells = input.lines()
            .flat_map(|l| l.chars().map(|c| if c == '#' { Cell::Alive } else { Cell::Dead }))
            .collect_vec();

        Universe {
            width,
            height,
            cells: cells.clone(),
            back_buffer: cells,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                match cell {
                    Cell::Dead => {
                        f.write_char('.')?
                    }
                    Cell::Alive => {
                        f.write_char('#')?
                    }
                };
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() {
    let initial = include_str!("../inputs/2015/day18/input.txt");

    let mut universe = Universe::new(100, 100, initial);
    for _ in 0..100 { universe.tick() }
    println!("{}", universe.count_alive());

    let mut universe = Universe::new(100, 100, initial);
    for _ in 0..100 { universe.tick_stuck() }
    println!("{}", universe.count_alive());
}