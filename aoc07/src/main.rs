use std::fs;
use std::collections::HashMap;
use regex::Regex;

type Bags = HashMap<String, Vec<(u32, String)>>;

fn holds_shiny_gold(bags: &Bags, bag: &str) -> bool {
    if let Some(kids) = bags.get(bag) {
        for (_, kid) in kids {
            if kid == "shiny gold" || holds_shiny_gold(bags, kid) {
                return true;
            }
        }
    }
    false
}

fn required_total(bags: &Bags, bag: &str) -> u32 {
    let mut n = 1;
    if let Some(kids) = bags.get(bag) {
        for (num, kid) in kids {
            n += num * required_total(bags, kid);
        }
    }
    n
}

fn main() {
    let bag_re = Regex::new(r"^(\w+ \w+) bag").unwrap();
    let kid_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let mut bags: Bags = HashMap::new();

    for line in fs::read_to_string("input").unwrap().lines() {
        let bag = bag_re.captures(line).unwrap().get(1).unwrap().as_str();
        for cap in kid_re.captures_iter(line) {
            let num = cap[1].parse().unwrap();
            let kid = cap.get(2).unwrap().as_str();
            bags.entry(bag.to_owned()).or_default().push((num, kid.to_owned()));
        }
    }

    let mut n = 0;
    for bag in bags.keys() {
        if holds_shiny_gold(&bags, bag) {
            n += 1;
        }
    }
    println!("{} bag colors hold a shiny gold bag", n);

    let required_inside = required_total(&bags, "shiny gold") - 1;
    println!("{} bags are required inside a shiny gold bag", required_inside);
}
