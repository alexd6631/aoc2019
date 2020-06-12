use itertools::Itertools;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn new(name: String, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Ingredient { name, capacity, durability, flavor, texture, calories }
    }

    pub fn props(&self) -> [i32; 4] {
        [self.capacity, self.durability, self.flavor, self.texture]
    }
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    input.lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(":").collect_vec();
            let name = parts[0];
            let props: Vec<&str> = parts[1].split(",").collect_vec();
            let props = props.iter()
                .map(|p| p.trim().split_whitespace().nth(1).unwrap())
                .map(|p| p.parse::<i32>().unwrap())
                .collect_vec();

            Ingredient::new(
                name.to_owned(),
                props[0], props[1], props[2], props[3], props[4],
            )
        }).inspect(|p| println!("{:?}", p)).collect_vec()
}

type Coefficients = [i32; 4];

fn coefficients(n: i32) -> impl Itertools<Item=Coefficients> {
    (0..=n).flat_map(move |a| (0..=n - a)
        .flat_map(move |b| (0..=n - a - b)
            .map(move |c| [a, b, c, n - a - b - c])))
}

fn score(ingredients: &[Ingredient], coefficients: &Coefficients) -> i32 {
    let ingredients_props = ingredients.iter().map(|i| i.props()).collect_vec();

    (0..4)
        .map(|i| {
            let total = ingredients_props.iter()
                .map(move |p| p[i])
                .zip(coefficients)
                .map(|(a, b)| a * b)
                .sum();

            if total >= 0 { total } else { 0 }
        }).fold(1, |acc, i| acc * i)
}

fn calories(ingredients: &[Ingredient], coefficients: &Coefficients) -> i32 {
    ingredients.iter()
        .map(|p| p.calories)
        .zip(coefficients)
        .map(|(a, b)| a * b)
        .sum()
}

fn solve_part_1(ingredients: &[Ingredient]) {
    let res = coefficients(100).map(|c| score(ingredients, &c)).max().unwrap();
    println!("{}", res)
}

fn solve_part_2(ingredients: &[Ingredient]) {
    let res = coefficients(100)
        .filter(|c| calories(ingredients, c) == 500)
        .map(|c| score(ingredients, &c)).max().unwrap();
    println!("{}", res)
}

fn main() {
    let input = include_str!("../inputs/2015/day15.txt");

    let ingredients = parse_ingredients(input);
    solve_part_1(&ingredients);
    solve_part_2(&ingredients);
}