use std::fs;
use itertools::Itertools;

fn main() {
    let ns: Vec<i32> = fs::read_to_string("input").unwrap().lines()
        .map(|x| x.parse().unwrap()).collect();

    for n in ns.iter().combinations(2) {
        if n[0] + n[1] == 2020 {
            println!("Product of two: {}", n[0] * n[1]);
        }
    }

    for n in ns.iter().combinations(3) {
        if n[0] + n[1] + n[2] == 2020 {
            println!("Product of three: {}", n[0] * n[1] * n[2]);
        }
    }
}
