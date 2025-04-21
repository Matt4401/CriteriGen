use colored::*;

mod parsing;

use parsing::retrieve_input::ask_function::get_string;
use parsing::verify_function::create_del_cfile::{create_function_file, delete_folder_recursively};
use parsing::verify_function::is_a_function::looks_like_c_function;
use parsing::verify_function::is_function_coding_style::exec_cs;
use parsing::verify_function::write_in_file::write_to_file;

fn is_function_valid(function: &str) -> bool {
    if function.trim().is_empty() {
        eprintln!("{}", format!("Error the file is empty").red());
        return false;
    }

    if looks_like_c_function(function) == false {
        eprintln!(
            "{}",
            format!("Error the input is not a valid C function").red()
        );
        return false;
    }
    if exec_cs().is_err() {
        return false;
    }
    true
}

fn main() {
    let function =
        get_string("Enter the function in the function_to_test.txt and press \"Enter\":");
    if let Err(error) = create_function_file() {
        eprintln!(
            "{}",
            format!("Erreur lors de la création du fichier : {}", error).red()
        );
    }
    if let Err(error) = write_to_file("check_coding_style/function.c", &function) {
        eprintln!(
            "{}",
            format!("Erreur lors de l'écriture dans le fichier : {}", error).red()
        );
    }
    if is_function_valid(&function) == false {
        eprintln!(
            "{}",
            format!(
                "{}{}",
                "Error: the given function is not valid".red().bold(),
                ", pay attention to the coding style banana"
            )
            .red()
        );
    } else {
        println!("{}", "The function is valid.".green());
    }
    if let Err(error) = delete_folder_recursively("check_coding_style") {
        eprintln!(
            "{}",
            format!("Erreur lors de la suppression du dossier : {}", error).red()
        );
    }
}
