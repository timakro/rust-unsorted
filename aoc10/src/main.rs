use std::fs;
use itertools::Itertools;

fn main() {
    let mut ns = vec![0];
    ns.extend(fs::read_to_string("input").unwrap().lines()
              .map(|x| x.parse::<u32>().unwrap()));
    ns.sort();
    ns.push(ns.last().unwrap() + 3);

    let mut num_of_1s = 0;
    let mut num_of_3s = 0;
    for (prev, next) in ns.iter().tuple_windows() {
        let diff = next - prev;
        if diff == 1 {
            num_of_1s += 1;
        } else if diff == 3 {
            num_of_3s += 1;
        } else {
            panic!("Did not expect difference of {}", diff);
        }
    }
    println!("1s multiplied by 3s is {}", num_of_1s * num_of_3s);

    let calc_n = ns.len() - 1;
    let mut arrangements: Vec<u64> = vec![0; calc_n];
    arrangements.push(1);
    for i in (0..calc_n).rev() {
        let mut j = i + 1;
        while j < ns.len() && ns[j] - ns[i] <= 3 {
            arrangements[i] += arrangements[j];
            j += 1;
        }
    }
    println!("{} possible arrangements", arrangements[0]);

}
