use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn read_fasta(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('>') {
            sequence.push_str(&line);
        }
    }

    Ok(sequence)
}