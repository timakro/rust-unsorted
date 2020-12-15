use std::collections::HashMap;

const INPUT: &str = "7,14,0,17,11,1,2";

fn main() {
    let mut ns = INPUT.split(",").map(|x| x.parse::<u32>().unwrap());
    let mut last_seen = HashMap::new();
    let mut turn: u32 = 1;
    let mut last_n = 0;
    while turn <= 30000000 {
        let n = ns.next().unwrap_or_else(|| {
            match last_seen.get(&last_n) {
                Some(last_n_turn) => turn - last_n_turn,
                _                 => 0,
            }
        });
        if turn == 2020 || turn == 30000000 {
            println!("The {}th number spoken is {}", turn, n);
        }
        last_seen.insert(last_n, turn);
        last_n = n;
        turn += 1;
    }
}
