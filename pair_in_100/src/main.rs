use colored::Colorize;
use std::thread;
use std::time::Duration;

fn main() {
    for numero in 1..100 {
        if numero % 2 == 0 {

            println!("{}", numero.to_string().bold().truecolor(128, 0, 255).underline());
            thread::sleep(Duration::from_millis(200));

        }
    }
}
