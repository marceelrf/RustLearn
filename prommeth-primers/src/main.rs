use colored::Colorize;
use std::thread;
use std::time::Duration;

fn main() {

    println!("Welcome to {}-{}.", "prommeth".blue().bold().italic(), "primers".red().italic().bold());
    println!("");
    println!("");
    println!("{}:", "Usage".truecolor(224,111,0).bold());//Orange color
    println!("  SayDeb1");//exemplo de um subprograma
    println!("  SayDeb2");//exemplo de um subprograma
    println!("    -h, --help");
    println!("    -v, --version");
    println!("");
    println!("By {}","Marcel Ferreira".truecolor(128,229,255).bold().italic());
}

fn wait2printDeb() {
    thread::sleep(Duration::from_millis(200));//wait two seconds!

}