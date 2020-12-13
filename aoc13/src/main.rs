use std::fs;
use num::integer::lcm;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    let earliest: u64 = lines.next().unwrap().parse().unwrap();

    let mut best_wait = u64::MAX;
    let mut best_id   = 0;

    let mut timestamp = 0;
    let mut multiple  = 1;

    for (i, id) in lines.next().unwrap().split(",").enumerate() {
        let i = i as u64;
        if let Ok(id) = id.parse::<u64>() {
            let wait = id - earliest%id;
            if wait < best_wait {
                best_wait = wait;
                best_id = id;
            }

            while (timestamp + i)%id != 0 {
                timestamp += multiple;
            }
            multiple = lcm(multiple, id);
        }
    }

    println!("Best bus ID multiplied by wait time is {}", best_id * best_wait);
    println!("Beginning with {} all buses depart at their offsets", timestamp);
}
