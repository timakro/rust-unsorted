use std::fs;
use std::collections::{VecDeque, HashSet};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Decks {
    p1: VecDeque<usize>,
    p2: VecDeque<usize>,
}

struct Result {
    winner: u8,
    decks: Decks,
}

fn calc_score(result: &Result) -> usize {
    let deck = if result.winner == 1 { &result.decks.p1 }
                                else { &result.decks.p2 };
    deck.iter().rev().zip(1..).map(|(c, f)| c * f).sum() 
}

fn combat(mut decks: Decks) -> Result {
    loop {
        if decks.p1.is_empty() {
            return Result { winner: 2, decks };
        }
        if decks.p2.is_empty() {
            return Result { winner: 1, decks };
        }

        let c1 = decks.p1.pop_front().unwrap();
        let c2 = decks.p2.pop_front().unwrap();

        assert!(c1 != c2);

        if c1 > c2 {
            decks.p1.push_back(c1);
            decks.p1.push_back(c2);
        } else {
            decks.p2.push_back(c2);
            decks.p2.push_back(c1);
        }
    }
}

fn recursive_combat(mut decks: Decks) -> Result {
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&decks) {
            return Result { winner: 1, decks };
        }
        seen.insert(decks.clone());

        if decks.p1.is_empty() {
            return Result { winner: 2, decks };
        }
        if decks.p2.is_empty() {
            return Result { winner: 1, decks };
        }

        let c1 = decks.p1.pop_front().unwrap();
        let c2 = decks.p2.pop_front().unwrap();

        let winner = if decks.p1.len() >= c1 && decks.p2.len() >= c2 {
            recursive_combat(Decks {
                p1: decks.p1.iter().copied().take(c1).collect(),
                p2: decks.p2.iter().copied().take(c2).collect(),
            }).winner
        } else {
            assert!(c1 != c2);
            if c1 > c2 { 1 } else { 2 }
        };

        if winner == 1 {
            decks.p1.push_back(c1);
            decks.p1.push_back(c2);
        } else {
            decks.p2.push_back(c2);
            decks.p2.push_back(c1);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut blocks = input.split("\n\n");
    let mut decks = Decks { p1: VecDeque::new(), p2: VecDeque::new() };

    for line in blocks.next().unwrap().lines().skip(1) {
        decks.p1.push_back(line.parse().unwrap());
    }
    for line in blocks.next().unwrap().lines().skip(1) {
        decks.p2.push_back(line.parse().unwrap());
    }

    let result = combat(decks.clone());
    println!("Player {} wins combat with a score of {}",
             result.winner, calc_score(&result));

    let result = recursive_combat(decks);
    println!("Player {} wins recursive combat with a score of {}",
             result.winner, calc_score(&result));
}
