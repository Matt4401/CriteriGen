use colored::*;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

fn execute_coding_style_check() -> io::Result<()> {
    let output = Command::new("coding-style")
        .arg("check_coding_style/")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        eprintln!("Coding style check failed.");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Coding style check failed.",
        ));
    }

    Ok(())
}

fn is_file_empty(file_path: &str) -> Option<bool> {
    match fs::metadata(file_path) {
        Ok(metadata) => Some(metadata.len() == 0),
        Err(_) => None,
    }
}

fn delete_file(file_path: &str) -> io::Result<()> {
    if fs::remove_file(file_path).is_err() {
        return Ok(());
    }
    Ok(())
}

pub fn exec_cs() -> io::Result<()> {
    if let Err(error) = execute_coding_style_check() {
        eprintln!(
            "{}",
            format!("Error during coding style check: {}", error).red()
        );
        return Err(error);
    }
    let file_path = "coding-style-reports.log";
    if is_file_empty(file_path) == Some(false) {
        eprintln!(
            "{}",
            "The given function cannot be evaluated because it does not respect banana style coding."
                .red()
        );
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The given function cannot be evaluated because it does not respect banana style coding.",
        ));
    }
    delete_file(file_path)?;
    Ok(())
}
