use std::fs;
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let mut queue = VecDeque::new();
    let ns: Vec<u64> = fs::read_to_string("input").unwrap().lines()
                       .map(|x| x.parse().unwrap()).collect();

    for &n in &ns[..25] {
        queue.push_back(n);
    }

    let mut invalid_n = None;
    for &n in &ns[25..] {
        let mut is_sum = false;
        for (a, b) in queue.iter().tuple_combinations() {
            if a + b == n {
                is_sum = true;
                break;
            }
        }
        if !is_sum {
            println!("{} is not a sum", n);
            invalid_n = Some(n);
        }
        queue.pop_front();
        queue.push_back(n);
    }
    let invalid_n = invalid_n.unwrap();

    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    while end < ns.len() {
        if sum < invalid_n {
            sum += ns[end];
            end += 1;
        } else if sum > invalid_n {
            sum -= ns[start];
            start += 1;
        } else {
            let min = ns[start..end].iter().min().unwrap();
            let max = ns[start..end].iter().max().unwrap();
            println!("Found region with answer {}", min + max);
            break;
        }
    }
}
