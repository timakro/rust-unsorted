use std::fs;

struct PartOne {
    pos: (i32, i32),
    dir: i32, // North 3, East 2, South 1, West 0
}

impl PartOne {
    fn take(&mut self, action: &str, value: i32) {
        match action {
            "N" => self.pos.1 += value,
            "S" => self.pos.1 -= value,
            "E" => self.pos.0 += value,
            "W" => self.pos.0 -= value,
            "L" => self.dir = (self.dir + value/90).rem_euclid(4),
            "R" => self.dir = (self.dir - value/90).rem_euclid(4),
            "F" => if self.dir % 2 == 0 { self.pos.0 += value * (self.dir-1) }
                                   else { self.pos.1 += value * (self.dir-2) },
            _   => panic!("Unknown action {}", action)
        }
    }
}

struct PartTwo {
    pos: (i32, i32),
    wpp: (i32, i32),
}

impl PartTwo {
    fn take(&mut self, action: &str, value: i32) {
        match action {
            "N" => self.wpp.1 += value,
            "S" => self.wpp.1 -= value,
            "E" => self.wpp.0 += value,
            "W" => self.wpp.0 -= value,
            "L" => for _ in 0..value/90 { self.wpp = (-self.wpp.1, self.wpp.0) },
            "R" => for _ in 0..value/90 { self.wpp = (self.wpp.1, -self.wpp.0) },
            "F" => { self.pos.0 += value * self.wpp.0;
                     self.pos.1 += value * self.wpp.1 },
            _   => panic!("Unknown action {}", action)
        }
    }
}

fn main() {
    let mut one = PartOne { pos: (0, 0), dir: 2 };
    let mut two = PartTwo { pos: (0, 0), wpp: (10, 1) };

    for line in fs::read_to_string("input").unwrap().lines() {
        let (action, value) = line.split_at(1);
        let value: i32 = value.parse().unwrap();

        one.take(action, value);
        two.take(action, value);
    }

    println!("{} with old rules", one.pos.0.abs() + one.pos.1.abs());
    println!("{} with waypoint",  two.pos.0.abs() + two.pos.1.abs());
}
