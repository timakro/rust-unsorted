use std::fs;
use std::collections::HashSet;

fn main() {
    let mut ids = HashSet::new();

    for line in fs::read_to_string("input").unwrap().lines() {
        let row = line[..7].replace("F", "0").replace("B", "1");
        let col = line[7..].replace("L", "0").replace("R", "1");
        let row = u32::from_str_radix(&row, 2).unwrap();
        let col = u32::from_str_radix(&col, 2).unwrap();
        ids.insert(row * 8 + col);
    }

    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();
    println!("The highest seat ID is {}", max);
    for i in min..max {
        if !ids.contains(&i) {
            println!("{} is missing", i);
        }
    }
}
