use std::path::Path;
use clap::Parser;

/// CLI tool to get the file extension
#[derive(Parser)]
struct Cli {
    /// The path to the file
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Extrai a extensão do arquivo
    if let Some(extension) = get_file_extension(&args.path) {
        println!("A extensão do arquivo é: {}", extension);
    } else {
        println!("O arquivo não possui uma extensão.");
    }
}

fn get_file_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}