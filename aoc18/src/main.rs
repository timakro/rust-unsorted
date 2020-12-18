use std::fs;

fn process(ops: &mut Vec<char>, ns: &mut Vec<u64>, proc_mult: bool) {
    while let Some(&op) = ops.last() {
        if !(op == '+' || (proc_mult && op == '*')) {
            break;
        }
        let (n1, n2) = (ns.pop().unwrap(), ns.pop().unwrap());
        ns.push(match op {
            '+' => n1 + n2, '*' => n1 * n2,
            _ => unreachable!()
        });
        ops.pop();
    }
}

fn eval(expr: &str, precedence: bool) -> u64 {
    let mut ops = Vec::new();
    let mut ns  = Vec::new();
    for token in expr.chars() {
        match token {
            '0'..='9' => {
                ns.push(u64::from(token.to_digit(10).unwrap()));
            }
            '+' | '*' => {
                process(&mut ops, &mut ns, token == '*' || !precedence);
                ops.push(token);
            }
            '(' => {
                ops.push('(');
            }
            ')' => {
                process(&mut ops, &mut ns, true);
                assert!(*ops.last().unwrap() == '(');
                ops.pop();
            }
            _ => ()
        }
    }
    process(&mut ops, &mut ns, true);
    ns.pop().unwrap()
}

fn main() {
    let mut n1 = 0;
    let mut n2 = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
        n1 += eval(line, false);
        n2 += eval(line, true);
    }
    println!("{} without operator precedence", n1);
    println!("{} with opposite operator precedence", n2);
}
