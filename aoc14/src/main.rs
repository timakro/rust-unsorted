use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn set(mem2: &mut HashMap<u64, u64>, addr: u64, val: u64, mask_x: u64, i: u8) {
    for i in i..36 {
        if mask_x & (1<<i) > 0 {
            set(mem2, addr & !(1<<i), val, mask_x, i+1);
            set(mem2, addr |  (1<<i), val, mask_x, i+1);
            return;
        }
    }
    mem2.insert(addr, val);
}

fn main() {
    let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut mask_0 = 0;
    let mut mask_1 = 0;
    let mut mask_x = 0;
    let mut mem1 = HashMap::new();
    let mut mem2 = HashMap::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            mask_0 = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
            mask_1 = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            mask_x = u64::from_str_radix(&mask.replace("1", "0")
                                              .replace("X", "1"), 2).unwrap();
        } else if let Some(caps) = mem_re.captures(line) {
            let addr: u64 = caps[1].parse().unwrap();
            let val:  u64 = caps[2].parse().unwrap();
            mem1.insert(addr, (val & mask_0) | mask_1);
            set(&mut mem2, addr | mask_1, val, mask_x, 0);
        }
    }
    println!("Total of {} for version 1", mem1.values().sum::<u64>());
    println!("Total of {} for version 2", mem2.values().sum::<u64>());
}
