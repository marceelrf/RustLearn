use colored::Colorize;
//use std::thread;
//use std::time::Duration;

fn main() {

    print_help();
}


fn print_help() {
    //thread::sleep(Duration::from_millis(200));//wait two seconds!
    println!("Welcome to {}-{}.", "prommeth".blue().bold().italic(), "primers".red().italic().bold());
    println!("");
    println!("");
    println!("{}:", "Usage".truecolor(224,111,0).bold());//Orange color
    print!("prommeth-primers [OPTION] ..",);
    println!("");
    println!("");
    println!("Flags:");
    println!("    -h, --help");
    println!("    -v, --version");
    println!("Options: ");
    println!("  CpG");//exemplo de um subprograma
    println!("  Dimer");//exemplo de um subprograma
    println!("");
    println!("By {}","Marcel Ferreira".truecolor(128,229,255).bold().italic());

}