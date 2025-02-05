use std::io;
use colored::Colorize;

fn main() {
    println!("Please, enter the weight in kilograms (Kg):");
    let mut input_peso = String::new();
    io::stdin().read_line(&mut input_peso).expect("Error");
    let peso: f64 = input_peso.trim().parse().expect("Error");


    println!("Please, enter the height in meters (m):");
    let mut input_altura = String::new();
    io::stdin().read_line(&mut input_altura).expect("Error");
    let altura: f64 = input_altura.trim().parse().expect("Error");

    let imc = peso/(altura * altura);

    println!("The body mass index is {}.", imc.to_string().red().bold().underline())
}
