use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let rule_re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let input = fs::read_to_string("input").unwrap();
    let mut blocks = input.split("\n\n");

    let mut rules: Vec<(String, u32, u32, u32, u32)> = Vec::new();
    for rule in blocks.next().unwrap().lines() {
        let caps = rule_re.captures(rule).unwrap();
        rules.push((caps[1].to_owned(),
                    caps[2].parse().unwrap(), caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(), caps[5].parse().unwrap()));
    }

    let mut my_options: Vec<HashSet<&str>> = Vec::new();
    let mut my_values: Vec<u64> = Vec::new();
    for value in blocks.next().unwrap().lines().nth(1).unwrap().split(",") {
        let value: u64 = value.parse().unwrap();
        let mut options = HashSet::new();
        for (name, ..) in &rules {
            options.insert(name.as_str());
        }
        my_options.push(options);
        my_values.push(value);
    }

    let mut invalid_acc = 0;
    for ticket in blocks.next().unwrap().lines().skip(1) {
        for (value, options) in ticket.split(",").zip(&mut my_options) {
            let value: u32 = value.parse().unwrap();
            let mut matching = HashSet::new();
            for (name, l1, u1, l2, u2) in &rules {
                if (*l1 <= value && value <= *u1) ||
                   (*l2 <= value && value <= *u2) {
                    matching.insert(name.as_str());
                }
            }
            if matching.is_empty() {
                invalid_acc += value;
            } else {
                options.retain(|x| matching.contains(x));
            }
        }
    }
    println!("Scanning error rate: {}", invalid_acc);

    let mut to_remove: Vec<&str> = my_options.iter().filter(|x| x.len() == 1)
                                  .map(|x| *x.iter().nth(0).unwrap()).collect();
    while let Some(rem) = to_remove.pop() {
        for options in my_options.iter_mut().filter(|x| x.len() > 1) {
            options.remove(rem);
            if options.len() == 1 {
                to_remove.push(options.iter().nth(0).unwrap());
            }
        }
    }

    let mut departure_mul: u64 = 1;
    for (value, options) in my_values.iter().zip(&my_options) {
        assert!(options.len() == 1);
        let name = options.iter().nth(0).unwrap();
        if name.starts_with("departure ") {
            departure_mul *= value;
        }
    }
    println!("Departure fields multiplied: {}", departure_mul);
}
