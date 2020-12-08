use std::fs;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Copy, Clone)]
enum Instr {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

enum SimResult {
    Term(i32),
    Loop(i32),
}

impl Instr {
    fn from(s: &str) -> Instr {
        if let Some(s) = s.strip_prefix("acc ") {
            Instr::ACC(s.parse().unwrap())
        } else if let Some(s) = s.strip_prefix("jmp ") {
            Instr::JMP(s.parse().unwrap())
        } else if let Some(s) = s.strip_prefix("nop ") {
            Instr::NOP(s.parse().unwrap())
        } else {
            panic!("Invalid instruction: {}", s);
        }
    }
}

fn simulate(instr: &Vec<Instr>) -> SimResult {
    let mut visited = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;

    loop {
        if usize::try_from(pc).unwrap() == instr.len() {
            return SimResult::Term(acc);
        }
        if visited.contains(&pc) {
            return SimResult::Loop(acc);
        }
        visited.insert(pc);
        match instr[usize::try_from(pc).unwrap()] {
            Instr::ACC(n) => { pc += 1; acc += n; }
            Instr::JMP(n) => { pc += n; }
            Instr::NOP(_) => { pc += 1; }
        };
    }
}

fn main() {
    let mut instr: Vec<_> = fs::read_to_string("input").unwrap().lines()
                            .map(|l| Instr::from(l)).collect();

    if let SimResult::Loop(acc) = simulate(&instr) {
        println!("Found loop, accumulator is {}", acc);
    } 
    
    for i in 0..instr.len() {
        let old = instr[i];
        instr[i] = match instr[i] {
            Instr::JMP(n) => Instr::NOP(n),
            Instr::NOP(n) => Instr::JMP(n),
            _ => continue,
        };

        if let SimResult::Term(acc) = simulate(&instr) {
            println!("Found solution, accumulator is {}", acc);
        }

        instr[i] = old;
    }
}
