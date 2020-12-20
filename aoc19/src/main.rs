use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(PartialEq, Eq, Hash, Debug)]
enum RightSide {
    Terminal(char),
    NonTerminals(u32, u32),
}

fn count_matching(rules: &HashMap<RightSide, HashSet<u32>>, words: &str) -> u32 {
    let mut n = 0;
    for word in words.lines() {
        let mut v: HashMap<(u32,u32), HashSet<u32>> = HashMap::new();
        for (i, c) in (0..).zip(word.chars()) {
            let ns = rules.get(&RightSide::Terminal(c)).unwrap();
            v.entry((i,1)).or_default().extend(ns);
        }

        let len: u32 = word.chars().count().try_into().unwrap();
        for j in 2..=len {
            for i in 0..=len-j {
                let mut new: HashSet<u32> = HashSet::new();
                for k in 1..j {
                    for &n1 in v.get(&(i,k)).unwrap() {
                        for &n2 in v.get(&(i+k,j-k)).unwrap() {
                            let right = RightSide::NonTerminals(n1, n2);
                            if let Some(ns) = rules.get(&right) {
                                new.extend(ns);
                            }
                        }
                    }
                }
                v.entry((i,j)).or_default().extend(new);
            }
        }
        if v.get(&(0,len)).unwrap().contains(&0) {
            n += 1;
        }
    }
    n
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut blocks = input.split("\n\n");
    let mut rules: HashMap<RightSide, HashSet<u32>> = HashMap::new();
    let mut single_rules: Vec<(u32,u32)> = Vec::new();
    let mut unused = 1000;

    for line in blocks.next().unwrap().lines() {
        let colon_i = line.find(':').unwrap();
        let left: u32 = line[..colon_i].parse().unwrap();
        for option in line[colon_i+1..].split('|') {
            let chars: Vec<char> = option.trim().chars().collect();
            if chars.len() == 3 && chars[0] == '"' && chars[2] == '"' {
                let right = RightSide::Terminal(chars[1]);
                rules.entry(right).or_default().insert(left);
                continue;
            }

            let mut ns: Vec<u32> = option.split_whitespace()
                                   .map(|x| x.parse().unwrap()).collect();
            if ns.len() == 1 {
                single_rules.push((left, ns[0]));
                continue;
            }

            while ns.len() > 2 {
                let (n2, n1) = (ns.pop().unwrap(), ns.pop().unwrap());
                let right = RightSide::NonTerminals(n1, n2);
                rules.entry(right).or_default().insert(unused);
                ns.push(unused);
                unused += 1;
            }
            let right = RightSide::NonTerminals(ns[0], ns[1]);
            rules.entry(right).or_default().insert(left);
        }
    }

    for (left, n) in single_rules {
        for (_, l) in &mut rules {
            if l.contains(&n) {
                l.insert(left);
            }
        }
    }

    let words = blocks.next().unwrap();
    println!("{} messages are matching", count_matching(&rules, &words));

    rules.entry(RightSide::NonTerminals(42,8)).or_default().insert(8);
    rules.entry(RightSide::NonTerminals(11,31)).or_default().insert(unused);
    rules.entry(RightSide::NonTerminals(42,unused)).or_default().insert(11);

    println!("{} messages match after update", count_matching(&rules, &words));
}
