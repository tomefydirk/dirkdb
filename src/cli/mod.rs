use std::{
    fmt::Display,
    io::{self, Write},
};

use dialoguer::console::style;

use crate::{
     error_lib::SqlError, general_struct::constant::SEMICOLON_SIGN, parsing::select_parser::func::parse_select, tokenizer::scan_token
};

pub fn introduction() {
    println!(
        "\tBienvenue dans {} / Version : 1.0 ",
        style("dirkdb").dim()
    );
    println!(
        "
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Quo itaque consequatur
     veritatis iste optio temporibus alias, dolor amet in sunt ad delectus
      unde consequuntur reiciendis accusamus nemo. Mollitia, soluta non.
    "
    );
    println!("\n\tColaborateur : tomefydirk tony_mushah\n");
}

pub fn request_reader() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        print!("{}", style("|DirkDB> ").bold().italic().bright());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().eq_ignore_ascii_case("QUIT") {
            return Ok(());
        }

        handle_multi_request(&buffer);
    }
}
fn print_erreur<T: Display>(statues: &str, e: &T) {
    println!(
        "{}[{}{statues}] ({e})",
        style("Erreur").bold().dim(),
        style("#").bold().dim()
    )
}
fn handle_multi_request(input: &str) {
    while let Ok(next_input) = ask_request(input) {
        todo!()
    }
}
fn ask_request(input: &str) -> Result<&str, SqlError<String>> {
    let b = parse_select(input);
    match b {
        Ok(result) => match result.1.eval() {
            Ok(a) => println!("{:?}", a),
            Err(e) => print_erreur("evaluation", &e),
        },
        Err(e) => print_erreur("parsing", &e),
    }

    println!();
    todo!()
}
