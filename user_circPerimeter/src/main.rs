use std::io;
use colored::Colorize;

fn main() {
    println!("Please enter the circle radius:");

    let mut input_radius = String::new();
    io::stdin().read_line(&mut input_radius).expect("Error");
    let radius: f64 = input_radius.trim().parse().expect("Error");

    let perimeter = 2.0 * std::f64::consts::PI * radius;

    println!("The circle perimeter is {}.", perimeter.to_string().bold().green().underline());
}
