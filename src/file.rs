use crate::stackforth::StackForth;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file = File::open(file_name)?;

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(texto) => lines.push(texto),
            Err(error) => eprintln!("Error al leer línea: {}", error),
        }
    }
    Ok(lines.join(" "))
}

pub fn write_to_file(file_name: &str, stack: &mut StackForth) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;

    let mut values = Vec::new();
    while let Ok(value) = stack.pop() {
        values.push(value.to_string());
    }

    if !values.is_empty() {
        values.reverse();
        write!(file, "{}", values.join(" "))?;
    }

    Ok(())
}
