use std::io;
use colored::Colorize;

fn main() {
    println!("Type your age:");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age).expect("Error.");
    let age: u32 = input_age.trim().parse().expect("Error.");


    if age >= 18 {
        println!("You are {}.", "DE MAIOR".red().bold());
    } else {
        println!("Your are {}.", "DE MENOR".cyan().bold());
    }
}
