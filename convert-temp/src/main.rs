use std::io;

fn main() {
    println!("Enter a temperature in any unit.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: f64 = input.trim().parse().expect("Not a number");

    let celsius = [
        input,
        (input - 32.) / 1.8, // Fahrenheit to Celsius
        input - 273.15       // Kelvin to Celsius
    ];

    println!("{:10} {:10} {:10}", "Celsius", "Fahrenheit", "Kelvin");
    for celsius in celsius.iter() {
        println!("{:10.2} {:10.2} {:10.2}",
            celsius,
            celsius * 1.8 + 32., // Celsius to Fahrenheit
            celsius + 273.15     // Celsius to Kelvin
        );
    }
}
