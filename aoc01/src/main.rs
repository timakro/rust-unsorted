use std::fs;
use itertools::Itertools;

fn main() {
    let ns: Vec<i32> = fs::read_to_string("input").unwrap().lines()
        .map(|x| x.parse().unwrap()).collect();

    for (a, b) in ns.iter().tuple_combinations() {
        if a + b == 2020 {
            println!("Product of two: {}", a * b);
        }
    }

    for (a, b, c) in ns.iter().tuple_combinations() {
        if a + b + c == 2020 {
            println!("Product of three: {}", a * b * c);
        }
    }
}
