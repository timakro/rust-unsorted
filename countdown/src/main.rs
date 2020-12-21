use std::collections::{HashSet,HashMap};
use std::sync::mpsc;
use std::thread;
use num_cpus;

fn calc_results(results: &mut HashSet<i32>, ns: Vec<i32>) {
    for i in 0..ns.len() {
        for j in i+1..ns.len() {
            let mut result = |r| {
                results.insert(r);
                let ns = [&ns[..i], &ns[i+1..j], &ns[j+1..], &[r]].concat();
                calc_results(results, ns);
            };

            let (s, l) = if ns[i] <= ns[j] { (ns[i], ns[j]) }
                                      else { (ns[j], ns[i]) };

            result(l + s);
            result(l * s);
            result(l - s);
            if s != 0 && l % s == 0 {
                result(l / s);
            }
        }
    }
}

fn get_all_boards() -> Vec<[i32; 6]> {
    let mut boards = Vec::new();
    for a in 1..=10 {
        for b in a..=10 {
            for c in b+i32::from(a==b)..=10 {
                for d in c+i32::from(b==c)..=10 {
                    for e in d+i32::from(c==d)..=10 {
                        for f in e+i32::from(d==e)..=10 {
                            boards.push([a,b,c,d,e,f]);
                        }
                        for i in (25..=100).step_by(25) {
                            boards.push([a,b,c,d,e,i]);
                        }
                    }
                    for i in (25..=100).step_by(25) {
                        for j in (i+25..=100).step_by(25) {
                            boards.push([a,b,c,d,i,j]);
                        }
                    }
                }
                for i in (25..=100).step_by(25) {
                    for j in (i+25..=100).step_by(25) {
                        for k in (j+25..=100).step_by(25) {
                            boards.push([a,b,c,i,j,k]);
                        }
                    }
                }
            }
            for i in (25..=100).step_by(25) {
                for j in (i+25..=100).step_by(25) {
                    for k in (j+25..=100).step_by(25) {
                        for l in (k+25..=100).step_by(25) {
                            boards.push([a,b,i,j,k,l]);
                        }
                    }
                }
            }
        }
    }
    boards
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let all_boards: Vec<[i32; 6]> = get_all_boards();
    for boards in all_boards.chunks(all_boards.len() / num_cpus::get()) {
        let boards = boards.to_owned();
        let tx = tx.clone();
        thread::spawn(move || {
            for board in boards {
                let mut results: HashSet<i32> = HashSet::new();
                calc_results(&mut results, board.to_vec());
                tx.send(results).unwrap();
            }
        });
    }
    drop(tx);

    let mut commonness: HashMap<i32,i32> = HashMap::new();
    for results in rx {
        for result in results {
            if 100 <= result && result <= 999 {
                *commonness.entry(result).or_insert(0) += 1;
            }
        }
    }

    let mut commonness: Vec<(i32,i32)> = commonness.into_iter().collect();
    commonness.sort_by_key(|x| (x.1, x.0));
    for (result, commonness) in commonness {
        println!("{} {}", result, commonness);
    }
}
