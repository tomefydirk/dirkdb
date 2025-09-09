use std::{
    fmt::Display,
    io::{self, Write}, time::Instant,
};

pub mod aff;

use dialoguer::console::style;
use crate::{cli::aff::PrettyTable, parsing::select_parser::func::parse_select};

pub fn introduction() {
    println!("{}\n", style("Welcome to DirkDB!").bold().cyan());
    println!("Version 1.0 (vanilla) — based in Rust 🦀");
    println!("Type your SQL commands and end them with ';'.");
    println!("Enter QUIT or EXIT to quit.\n");
}

pub fn request_reader() -> io::Result<()> {
    let mut current_query = String::new(); // requête en cours
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    loop {
        let mut buffer = String::new();

        io::stdout().flush().unwrap();

        // Prompt différent selon qu'on attend une continuation ou une nouvelle requête
        if current_query.is_empty() {
            print!("{}", style("|DirkDB> ").bold().bright());
        } else {
            print!("{}", style("...> ").bold().bright());
        }

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();

        if input.eq_ignore_ascii_case("QUIT") || input.eq_ignore_ascii_case("EXIT") {
            println!("{}", style("Bye 👋").bold().green());
            return Ok(());
        }

        // Ajouter la ligne lue à la requête courante
        current_query.push_str(input);
        current_query.push('\n');

        let mut temp = String::new();
        let mut chars = current_query.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quotes => {
                    in_single_quotes = !in_single_quotes;
                    temp.push(c);
                }
                '"' if !in_single_quotes => {
                    in_double_quotes = !in_double_quotes;
                    temp.push(c);
                }
                ';' if !in_single_quotes && !in_double_quotes => {
                    let query = temp.trim();
                    if !query.is_empty() {
                        ask_request(query);
                    }
                    temp.clear(); // réinitialiser après exécution
                }
                _ => temp.push(c),
            }
        }

        // Garder les caractères restants pour la prochaine ligne
        current_query = temp;
    }
}



/// Affiche une erreur formatée.
fn print_erreur<T: Display>(statues: &str, e: &T) {
    eprintln!(
        "{} {}[{}] {}",
        style("✖").red(),
        style("Erreur").bold().red(),
        style(statues).yellow(),
        e
    );
}

/// Parse et évalue une requête utilisateur.
fn ask_request(input: &str) {
   let start = Instant::now();

    match parse_select(input) {
        Ok(result) => match result.1.eval() {
            Ok(a) => {
                println!(
                    "{} {:#?}",
                    style("✔ Succès:").green().bold(),
                    start.elapsed()
                );
                let to_affiche = PrettyTable(&a);
                println!("\n{}", to_affiche);
            }
            Err(e) => print_erreur("evaluation", &e),
        },
        Err(e) => print_erreur("parsing", &e),
    }

    println!();
}
