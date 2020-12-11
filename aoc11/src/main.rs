use std::fs;

const DIRECTIONS: [(i32, i32); 8] =
    [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

fn count_adjacent(seating: &Vec<Vec<u8>>, r: i32, c: i32) -> u32 {
    let mut count = 0;
    for (dr, dc) in &DIRECTIONS {
        let tr = r + dr;
        let tc = c + dc;
        if tr >= 0 && (tr as usize) < seating.len() &&
           tc >= 0 && (tc as usize) < seating[0].len() &&
           seating[tr as usize][tc as usize] == b'#' {
            count += 1;
        }
    }
    count
}

fn count_visible(seating: &Vec<Vec<u8>>, r: i32, c: i32) -> u32 {
    let mut count = 0;
    for (dr, dc) in &DIRECTIONS {
        let mut tr = r + dr;
        let mut tc = c + dc;
        while tr >= 0 && (tr as usize) < seating.len() &&
              tc >= 0 && (tc as usize) < seating[0].len() {
            if seating[tr as usize][tc as usize] == b'#' {
                count += 1;
                break;
            } else if seating[tr as usize][tc as usize] == b'L' {
                break;
            }
            tr += dr;
            tc += dc;
        }
    }
    count
}

fn simulate(sim_seating: &mut Vec<Vec<u8>>,
            count: fn(&Vec<Vec<u8>>, i32, i32) -> u32,
            too_many: u32) -> u32 {
    loop {
        let mut changed = false;
        let seating = sim_seating.clone();
        for r in 0..seating.len() {
            for c in 0..seating[0].len() {
                let n = count(&seating, r as i32, c as i32);
                if seating[r][c] == b'L' && n == 0 {
                    sim_seating[r][c] = b'#';
                    changed = true;
                } else if seating[r][c] == b'#' && n >= too_many {
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

    println!("Part One {}", simulate(&mut seating.clone(), count_adjacent, 4));
    println!("Part Two {}", simulate(&mut seating.clone(), count_visible, 5));
}
