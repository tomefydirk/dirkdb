use std::{
    fmt::Display,
    io::{self, Write},
};

use dialoguer::console::style;

use crate::{
     parsing::select_parser::func::parse_select
};

pub fn introduction() {
    println!("Welcome to the DirkDB monitor.  Commands end with ;");
    println!("Your version is 1.0 vanilla\n");
    println!("Please if you have a problem due to this Version , signal Dirk Company with this link {}",style("https://github.com/tomefydirk/dirkdb").cyan());

    println!("\n\tColaborateur : tomefydirk tony_mushah\n");
}

pub fn request_reader() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        print!("{}", style("|DirkDB> ").bold().italic().bright());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().eq_ignore_ascii_case("QUIT") || buffer.trim().eq_ignore_ascii_case("EXIt"){
            println!("Bye");
            return Ok(());
        }

       ask_request(&buffer);
    }
}
fn print_erreur<T: Display>(statues: &str, e: &T) {
    println!(
        "{}[{}{statues}] ({e})",
        style("Erreur").bold().dim(),
        style("#").bold().dim()
    )
}
fn ask_request(input: &str) {
    let b = parse_select(input);
    match b {
        Ok(result) => match result.1.eval() {
            Ok(a) => println!("{:?}", a),
            Err(e) => print_erreur("evaluation", &e),
        },
        Err(e) => print_erreur("parsing", &e),
    }
    println!();
}
