use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn is_text_between(val: &str, lower: u32, upper: u32) -> bool {
    match val.parse::<u32>() {
        Ok(n)  => lower <= n && n <= upper,
        Err(_) => false
    }
}

fn is_valid_height(val: &str) -> bool {
    if let Some(s) = val.strip_suffix("cm") {
        is_text_between(s, 150, 193)
    } else if let Some(s) = val.strip_suffix("in") {
        is_text_between(s, 59, 76)
    } else {
        false
    }
}

fn main() {
    let passports = fs::read_to_string("input").unwrap();
    let hcl_re = Regex::new(r"^#[\da-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let mut n  = 0;
    let mut n2 = 0;

    for passport in passports.split("\n\n") {
        let mut seen = HashSet::new();
        let mut all_ok = true;

        for pass_field in passport.split_whitespace() {
            let mut splitter = pass_field.split(":");
            let key = splitter.next().unwrap();
            let val = splitter.next().unwrap();

            assert!(!seen.contains(key));

            let ok = match key {
                "byr" => is_text_between(val, 1920, 2002),
                "iyr" => is_text_between(val, 2010, 2020),
                "eyr" => is_text_between(val, 2020, 2030),
                "hgt" => is_valid_height(val),
                "hcl" => hcl_re.is_match(val),
                "ecl" => ecl_re.is_match(val),
                "pid" => pid_re.is_match(val),
                _ => continue
            };

            all_ok &= ok;
            seen.insert(key);
        }

        if seen.len() == 7 {
            n += 1;
            if all_ok {
                n2 += 1;
            }
        }
    }

    println!("{} passports complete", n);
    println!("{} passports valid", n2);
}
