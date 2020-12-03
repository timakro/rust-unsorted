use std::fs;

fn count_trees(rows: &Vec<&[u8]>, dx: usize, dy: usize) -> u32 {
    let width = rows[0].len();
    let mut trees = 0;
    for i in 0..rows.len()/dy {
        if rows[i*dy][i*dx % width] == b'#' {
            trees += 1;
        }
    }
    trees
}

fn main() {
    let rows = fs::read_to_string("input").unwrap();
    let rows: Vec<&[u8]> = rows.lines().map(|x| x.as_bytes()).collect();

    println!("Right 3, down 1: {}", count_trees(&rows, 3, 1));
    println!("Product: {}", count_trees(&rows, 1, 1)
                          * count_trees(&rows, 3, 1)
                          * count_trees(&rows, 5, 1)
                          * count_trees(&rows, 7, 1)
                          * count_trees(&rows, 1, 2));
}
