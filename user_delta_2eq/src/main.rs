use std::io;
use colored::Colorize;

fn main() {
    println!("The delta formula require A, B and C.");
    println!("Enter the value of A:");

    let mut input_A = String::new();
    io::stdin().read_line(&mut input_A).expect("Error. Bat format.");
    let A: f64 = input_A.trim().parse().expect("Error. Bat format.");

    println!("Enter the value of B:");

    let mut input_B = String::new();
    io::stdin().read_line(&mut input_B).expect("Error. Bat format.");
    let B: f64 = input_A.trim().parse().expect("Error. Bat format.");

    println!("Enter the value of C:");

    let mut input_C = String::new();
    io::stdin().read_line(&mut input_C).expect("Error. Bat format.");
    let C: f64 = input_C.trim().parse().expect("Error. Bat format.");

    let delta = B * B - 4.0 * A * C;

    println!("For A={}, B={} and C={} delta is {}.",A.to_string().cyan().bold(),
            B.to_string().cyan().bold(),
            C.to_string().cyan().bold(),
            delta.to_string().cyan().red().underline());
}
