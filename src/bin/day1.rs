

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let res = compute_total_mass(input, fuel_for_mass);
    println!("Part 1 : {}", res);

    let res = compute_total_mass(input, fuel_for_mass_advanced);
    println!("Part 2 : {}", res)
}

fn compute_total_mass<F>(input: &str, compute_mass: F) -> u64 where F : Fn(u32) -> u32 {
    input.lines()
        .map(|l| l.parse().unwrap())
        .map(compute_mass)
        .fold(0, |a, b| a + b as u64)
}

fn fuel_for_mass(mass: u32) -> u32 { mass / 3 - 2 }

fn fuel_for_mass_advanced(mass: u32) -> u32 {
    let q = mass / 3;
    if q > 2 {
        let r = q - 2;
        r + fuel_for_mass_advanced(r)
    } else { 0 }
}

mod tests {
    use crate::{fuel_for_mass, fuel_for_mass_advanced};

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100_756), 33583);
    }

    #[test]
    fn test_fuel_for_mass_advanced() {
        assert_eq!(fuel_for_mass_advanced(14), 2);
        assert_eq!(fuel_for_mass_advanced(1969), 966);
        assert_eq!(fuel_for_mass_advanced(100_756), 50346);
    }
}