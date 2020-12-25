use std::fs;
use std::iter;
use std::collections::HashSet;

const DIRS: [&str; 6] = ["e", "se", "sw", "w", "nw", "ne"];

fn go_dir(dir: &str, mut p: (i32, i32)) -> (i32, i32) {
    match dir {
        "e"  => { p.0 += 1 }
        "se" => { p.1 += 1; if p.1 % 2 == 0 { p.0 += 1 } }
        "sw" => { p.1 += 1; if p.1 % 2 != 0 { p.0 -= 1 } }
        "w"  => { p.0 -= 1 }
        "nw" => { p.1 -= 1; if p.1 % 2 != 0 { p.0 -= 1 } }
        "ne" => { p.1 -= 1; if p.1 % 2 == 0 { p.0 += 1 } }
        _    => ()
    }
    p
}

fn main() {
    let mut black = HashSet::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut char_it = line.chars();
        let mut p = (0, 0);
        while let Some(c1) = char_it.next() {
            let mut dir = c1.to_string();
            if c1 == 'n' || c1 == 's' {
                dir.push(char_it.next().unwrap());
            }
            p = go_dir(&dir, p);
        }
        if black.contains(&p) {
            black.remove(&p);
        } else {
            black.insert(p);
        }
    }
    println!("{} black tiles at the start", black.len());

    for _ in 0..100 {
        let old_black = black.clone();
        for &p in &old_black {
            for p in iter::once(p).chain(DIRS.iter().map(|d| go_dir(d, p))) {
                let mut neighbors = 0;
                for n in DIRS.iter().map(|d| go_dir(d, p)) {
                    if old_black.contains(&n) {
                        neighbors += 1;
                    }
                }
                if neighbors == 0 || neighbors > 2 {
                    black.remove(&p);
                } else if neighbors == 2 {
                    black.insert(p);
                }
            }
        }
    }
    println!("{} black tiles after 100 days", black.len());
}
