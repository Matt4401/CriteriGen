use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(
        file,
        "/*\n** EPITECH PROJECT, 2025\n** CriteriGen\n** File description:\n** function\n*/\n"
    )?;
    writeln!(file, "{}", content)?;
    Ok(())
}
