use colored::*;
use std::fs::{self, File};
use std::io::{self};

pub fn create_function_file() -> io::Result<()> {
    fs::create_dir_all("check_coding_style")?;
    let file_path = "check_coding_style/function.c";
    let _file = File::create(file_path)?;

    // println!(
    //     "{}",
    //     format!("Fichier {} créé dans check_coding_style !", file_path).green()
    // );
    Ok(())
}

pub fn delete_folder_recursively(path: &str) -> io::Result<()> {
    if fs::metadata(path).is_ok() {
        fs::remove_dir_all(path)?;
        // println!(
        //     "{}",
        //     format!("Le dossier '{}' a été supprimé avec succès.", path).green()
        // );
    } else {
        eprintln!("{}", format!("Le dossier '{}' n'existe pas.", path).red());
    }
    Ok(())
}
