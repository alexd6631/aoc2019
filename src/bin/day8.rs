use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct Layer {
    rows: Vec<Vec<u8>>
}

impl Layer {
    fn number_of_digit(&self, d: u8) -> usize {
        self.rows.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|n| **n == d).count()
        })
    }

    fn value_at(&self, row: u8, col: u8) -> u8 {
        self.rows[row as usize][col as usize]
    }
}

fn parse_layers(input: &str, width: u8, height: u8) -> Vec<Layer> {
    let bytes = input.as_bytes();
    let all_rows = bytes.chunks(width as usize)
        .map(|s| s.iter().map(|n| n - 48).collect_vec())
        .collect_vec();


    let layers = all_rows.chunks(height as usize)
        .map(|rows| Layer { rows: rows.to_vec() })
        .collect_vec();

    layers
}

fn solve_part_1(layers: &Vec<Layer>) -> usize {
    let layer = layers.iter()
        .min_by_key(|l| l.number_of_digit(0))
        .unwrap();
    layer.number_of_digit(1) * layer.number_of_digit(2)
}

fn solve_part_2(layers: &Vec<Layer>) -> String {
    let mut image = [[" "; 25]; 6];

    for row in 0 .. 6 {
        for col in 0 .. 25 {
            let color = color_at(layers, row, col);
            image[row as usize][col as usize] = if color == 0 { " " } else { "#" }
        }
    }

    image.iter().map(|l| l.join("")).join("\n")
}

fn color_at(layers: &Vec<Layer>, row: u8, col: u8) -> u8 {
    layers.iter()
        .map(|l| l.value_at(row, col))
        .find(|p| *p != 2)
        .unwrap_or(0)
}



fn main() {
    let layers = parse_layers(include_str!("../inputs/day8.txt"), 25, 6);
    let part_1 = solve_part_1(&layers);
    println!("{}", part_1);
    let part_2 = solve_part_2(&layers);
    println!("{}", part_2)

}

#[cfg(test)]
mod tests {
    use crate::{parse_layers, Layer, color_at};

    #[test]
    fn test_parse() {
        let layers = parse_layers("123456789012", 3, 2);
        assert_eq!(vec![
            Layer {
                rows: vec![vec![1, 2, 3], vec![4, 5, 6]]
            },
            Layer {
                rows: vec![vec![7, 8, 9], vec![0, 1, 2]]
            },
        ], layers)
    }

    #[test]
    fn test_foo() {
        let layers = parse_layers("0222112222120000", 2, 2);

        assert_eq!(0, color_at(&layers, 0, 0));
        assert_eq!(1, color_at(&layers, 0, 1));
        assert_eq!(1, color_at(&layers, 1, 0));
        assert_eq!(0, color_at(&layers, 1, 1));
    }
}