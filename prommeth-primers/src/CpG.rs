fn find_cpg_islands(sequence: &str, island_length: usize) -> Vec<usize> {
    let mut cpg_islands = Vec::new();
    let mut count = 0;

    for (i, window) in sequence.as_bytes().windows(2).enumerate() {
        if window == b"CG" {
            count += 1;
        } else {
            if count >= island_length {
                cpg_islands.push(i - count + 1);
            }
            count = 0;
        }
    }

    if count >= island_length {
        cpg_islands.push(sequence.len() - count);
    }

    cpg_islands
}

fn write_positions_to_file(positions: &[usize], output_path: &Path) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    for &position in positions {
        writeln!(file, "{}", position)?;
    }
    Ok(())
}

fn cpg(fasta_path: &Path, output_path: &Path, island_length: usize) -> io::Result<()> {
    let sequence = read_fasta(fasta_path)?;
    let cpg_islands = find_cpg_islands(&sequence, island_length);
    write_positions_to_file(&cpg_islands, output_path)?;
    Ok(())
}