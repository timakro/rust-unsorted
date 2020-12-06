use std::fs;
use std::collections::HashSet;

fn main() {
    let mut n = 0;
    let mut n2 = 0;
    for group in fs::read_to_string("input").unwrap().split("\n\n") {
        let mut union:        HashSet<u8> = HashSet::new();
        let mut intersection: HashSet<u8> = (b'a'..=b'z').collect();
        for line in group.lines() {
            let set = line.as_bytes().iter().copied().collect();
            union        = &union | &set;
            intersection = &intersection & &set;
        }
        n  += union.len();
        n2 += intersection.len();
    }
    println!("{} individual yeses", n);
    println!("{} consensual yeses", n2);
}
