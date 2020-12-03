use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)$").unwrap();
    let mut n = 0;
    let mut n2 = 0;

    for line in data.lines() {
        let caps = re.captures(&line).unwrap();
        let letter = caps[3].chars().next().unwrap();

        let occurrences = caps[4].matches(letter).count();
        let lower: usize = caps[1].parse().unwrap();
        let upper: usize = caps[2].parse().unwrap();
        if lower <= occurrences && occurrences <= upper {
            n += 1;
        }

        let first  = caps[4].chars().nth(lower - 1).unwrap();
        let second = caps[4].chars().nth(upper - 1).unwrap();
        if (first == letter) != (second == letter) {
            n2 += 1;
        }
    }

    println!("{} passwords compliant with old policy", n);
    println!("{} passwords compliant with new policy", n2)
}
