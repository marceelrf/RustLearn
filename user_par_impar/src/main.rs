use std::io;
use colored::Colorize;

fn main() {
    println!("Please, enter a number:");

    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).expect("Error.");
    let num: i32 = input_num.trim().parse().expect("Error.");

    if num % 2 == 0 {
        println!("Seu número é {}.", "PAR".red().bold().underline());
    } else {
        println!("Seu número é {}.", "ÍMPAR".green().bold().underline());
    }


}
