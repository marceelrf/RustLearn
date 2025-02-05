use std::io;
use colored::Colorize;


fn main() {
    println!("Please, enter the 1st number.");

    let num1: f64 = loop {
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Error.");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Wrong format."),
        }
    };

    println!("Please, enter the 2nd number.");

    let num2: f64 = loop {
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Error.");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Wrong format."),
        }
    };
    
    println!("Please, enter the 3rd number.");

    let num3: f64 = loop {
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Error.");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Wrong format."),
        }
    };

    let mean_val = (num1 * num2 * num3).powf(1.0/3.0);

    println!("The geometric mean is {}.", mean_val.to_string().red().bold().underline());

}
