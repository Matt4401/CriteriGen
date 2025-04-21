use colored::*;
use std::fs;
use std::io;

pub fn get_string(sentence: &str) -> String {
    println!("{}", sentence.blue().bold());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error during the input reading.");

    let function = fs::read_to_string("FUNCTION_TO_TEST.txt").unwrap_or_else(|_| {
        eprintln!("Error: Could not read the file 'FUNCTION_TO_TEST.txt'.");
        String::new()
    });

    function
}
