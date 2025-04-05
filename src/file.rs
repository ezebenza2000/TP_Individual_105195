use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn read_file(file_name: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_name)?;

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(texto) => lines.push(texto),
            Err(error) => eprintln!("Error al leer línea: {}", error),
        }
    }
    Ok(lines)
}

pub fn write_to_file(file_name: &str, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;

    writeln!(file, "{}", content)?;
    Ok(())
}
