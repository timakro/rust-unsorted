use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let ns: Vec<i32> = BufReader::new(File::open("input").unwrap())
        .lines().map(|x| x.unwrap().parse().unwrap()).collect();

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
