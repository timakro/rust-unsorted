use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let f = BufReader::new(File::open("input").unwrap());
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)$").unwrap();
    let mut n = 0;
    let mut n2 = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let chr = caps[3].chars().next().unwrap();
        let count = caps[4].matches(chr).count();
        let lb: usize = caps[1].parse().unwrap();
        let ub: usize = caps[2].parse().unwrap();
        if lb <= count && count <= ub {
            n += 1;
        }
        let lchr = caps[4].chars().nth(lb - 1).unwrap();
        let uchr = caps[4].chars().nth(ub - 1).unwrap();
        if (lchr == chr) != (uchr == chr) {
            n2 += 1;
        }
    }

    println!("{} passwords compliant with old policy", n);
    println!("{} passwords compliant with new policy", n2)
}
