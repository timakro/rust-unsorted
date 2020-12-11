use std::fs;

const DIRECTIONS: [(i32, i32); 8] =
    [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

fn count(seating: &Vec<Vec<u8>>, adjacent: bool, r: i32, c: i32) -> u32 {
    let mut count = 0;
    for (dr, dc) in &DIRECTIONS {
        let mut tr = r + dr;
        let mut tc = c + dc;
        while tr >= 0 && (tr as usize) < seating.len() &&
              tc >= 0 && (tc as usize) < seating[0].len() {
            let seat = seating[tr as usize][tc as usize];
            if seat == b'#' {
                count += 1;
                break;
            }
            if seat == b'L' || adjacent {
                break;
            }
            tr += dr;
            tc += dc;
        }
    }
    count
}

fn simulate(sim_seating: &mut Vec<Vec<u8>>, adjacent: bool,
            too_many: u32) -> u32 {
    loop {
        let mut changed = false;
        let seating = sim_seating.clone();
        for r in 0..seating.len() {
            for c in 0..seating[0].len() {
                let seat = seating[r][c];
                let n = count(&seating, adjacent, r as i32, c as i32);
                if seat == b'L' && n == 0 {
                    sim_seating[r][c] = b'#';
                    changed = true;
                } else if seat == b'#' && n >= too_many {
                    sim_seating[r][c] = b'L';
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    sim_seating.iter().flatten().map(|&s| if s == b'#' { 1 } else { 0 }).sum() 
}

fn main() {
    let seating: Vec<Vec<u8>> = fs::read_to_string("input").unwrap().lines()
                                .map(|x| x.to_owned().into_bytes()).collect();

    println!("{} for adjacent seats", simulate(&mut seating.clone(), true, 4));
    println!("{} for visible seats", simulate(&mut seating.clone(), false, 5));
}
