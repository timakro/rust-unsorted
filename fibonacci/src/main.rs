fn main() {
    println!("The 10th Fibonnaci number is {}", fibonacci(10));
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
