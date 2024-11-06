use std::io;

fn main() {
    loop {
        println!("Temperature Conversion Program");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit");
        println!("Please enter your choice (1, 2, or 3):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");
                let fahrenheit = read_temperature();
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);
            }
            2 => {
                println!("Enter temperature in Celsius:");
                let celsius = read_temperature();
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2}°C is {:.2}°F", celsius, fahrenheit);
            }
            3 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn read_temperature() -> f64 {
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    temp.trim().parse().expect("Please enter a valid number")
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
