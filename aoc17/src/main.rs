use std::fs;
use std::collections::HashSet;

fn step(cube: &mut HashSet<(i32,i32,i32,i32)>, ignore_4d: bool) {
    let old_cube = cube.clone();
    let mut checked = HashSet::new();
    for &(x,y,z,w) in &old_cube {
        for x in x-1..=x+1 {
            for y in y-1..=y+1 {
                for z in z-1..=z+1 {
                    for w in w-1..=w+1 {
                        if ignore_4d && w != 0 {
                            continue;
                        }
                        if checked.contains(&(x,y,z,w)) {
                            continue;
                        }
                        checked.insert((x,y,z,w));

                        let mut neighbors = 0;
                        for nx in x-1..=x+1 {
                            for ny in y-1..=y+1 {
                                for nz in z-1..=z+1 {
                                    for nw in w-1..=w+1 {
                                        if old_cube.contains(&(nx,ny,nz,nw)) {
                                            neighbors += 1;
                                        }
                                    }
                                }
                            }
                        }

                        if old_cube.contains(&(x,y,z,w)) {
                            neighbors -= 1;
                            if neighbors < 2 || neighbors > 3 {
                                cube.remove(&(x,y,z,w));
                            }
                        } else {
                            if neighbors == 3 {
                                cube.insert((x,y,z,w));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut cube = HashSet::new();
    for (x, line) in (0..).zip(fs::read_to_string("input").unwrap().lines()) {
        for (y, chr) in (0..).zip(line.chars()) {
            if chr == '#' {
                cube.insert((x,y,0,0));
            }
        }
    }
    let mut cube_3d = cube.clone();
    let mut cube_4d = cube;

    for _ in 0..6 {
        step(&mut cube_3d, true);
        step(&mut cube_4d, false);
    }

    println!("Active cubes in 3D: {}", cube_3d.len());
    println!("Active cubes in 4D: {}", cube_4d.len());
}
