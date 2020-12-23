use std::collections::HashMap;

const INPUT: &str = "476138259";

fn do_moves(init: &[u32], n: u32) -> HashMap<u32, u32> {
    let mut next = HashMap::new();
    for w in init.windows(2) {
        next.insert(w[0], w[1]);
    }
    next.insert(*init.last().unwrap(), init[0]);

    let max = *init.iter().max().unwrap();
    let mut current = init[0];
    for _ in 0..n {
        let n1 = *next.get(&current).unwrap();
        let n2 = *next.get(&n1).unwrap();
        let n3 = *next.get(&n2).unwrap();
        let n4 = *next.get(&n3).unwrap();

        let dest = (1..=max)
            .chain(1..current)
            .rev()
            .find(|&x| x != n1 && x != n2 && x != n3)
            .unwrap();

        next.insert(current, n4);
        next.insert(n3, *next.get(&dest).unwrap());
        next.insert(dest, n1);

        current = n4;
    }
    next
}

fn main() {
    let mut init = Vec::new();
    for chr in INPUT.chars() {
        if let Some(n) = char::to_digit(chr, 10) {
            init.push(n);
        }
    }

    let next = do_moves(&init, 100);
    let mut result = String::new();
    let mut i = *next.get(&1).unwrap();
    while i != 1 {
        result.push(std::char::from_digit(i, 10).unwrap());
        i = *next.get(&i).unwrap();
    }
    println!("Order for part one is {}", result);

    for i in 10..=1_000_000 {
        init.push(i);
    }
    let next = do_moves(&init, 10_000_000);
    let n1 = *next.get(&1).unwrap();
    let n2 = *next.get(&n1).unwrap();
    println!("Product for part two is {}", (n1 as u64) * (n2 as u64));
}
