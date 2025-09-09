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

/// Main loop to read user input.
pub fn request_reader() -> io::Result<()> {
    loop {
        let mut buffer = String::new();

        io::stdout().flush().unwrap();
        print!("{}", style("|DirkDB> ").bold().bright());
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().eq_ignore_ascii_case("QUIT")
            || buffer.trim().eq_ignore_ascii_case("EXIT")
        {
            println!("{}", style("Bye 👋").bold().green());
            return Ok(());
        }

        ask_request(&buffer);
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
