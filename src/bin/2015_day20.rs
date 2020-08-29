use num_integer::Roots;

fn compute_present(house_number: u32) -> u32 {
    (1 ..= house_number)
        .filter(|i| house_number % i == 0)
        .map(|i| i * 10).sum()
}

fn compute_present_opt(house_number: u32) -> u32 {
    let mut presents = 0;
    for i in 1 ..= house_number.sqrt() {
        if house_number % i == 0 {
            let q = house_number / i;
            if q == i {
                presents += i * 10
            } else {
                presents += (i * 10) + (q) * 10
            }
        }
    }
    presents
}

fn compute_present_part_2(house_number: u32) -> u32 {
    let mut presents = 0;
    for i in 1 ..= house_number.sqrt() {
        if house_number % i == 0 {
            let q = house_number / i;
            if q == i {
                if q <= 50 {
                    presents += i * 11
                }
            } else {
                if q <= 50 {
                    presents += i * 11
                }
                if i <= 50 {
                    presents += q * 11
                }
            }
        }
    }
    presents
}

fn main() {
    let (i, p) = (1..)
        .map(|i| (i, compute_present_opt(i)))
        .find(|(_, p)| *p >= 29000000)
        .unwrap();
    println!("house {}, {}", i, p);

    let (i, p) = (1..)
        .map(|i| (i, compute_present_part_2(i)))
        .find(|(_, p)| *p >= 29000000)
        .unwrap();
    println!("house {}, {}", i, p)
}