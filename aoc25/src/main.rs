use std::fs;
use std::collections::HashMap;

fn modular_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut x = 1;
    for _ in 0..exp {
        x = (x * base) % modulus;
    }
    x
}

fn discrete_log(x: u64, base: u64, modulus: u64) -> Option<u64> {
    let m = (modulus as f32).sqrt() as u64 + 1;
    let mut babies = HashMap::new();
    let mut baby = 1;
    for j in 0..m {
        babies.insert(baby, j);
        baby = (baby * base) % modulus;
    }
    let f = modular_pow(base, modulus - m - 1, modulus);
    let mut giant = x;
    for i in 0..m {
        if let Some(j) = babies.get(&giant) {
            return Some(i*m + j);
        }
        giant = (giant * f) % modulus;
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut line_it = input.lines();
    let p1: u64 = line_it.next().unwrap().parse().unwrap();
    let p2: u64 = line_it.next().unwrap().parse().unwrap();

    let l2 = discrete_log(p2, 7, 20201227).unwrap();
    println!("The key is {}", modular_pow(p1, l2, 20201227));
}
