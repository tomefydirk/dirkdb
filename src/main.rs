use std::io;

use dialoguer::console::style;
use dirkdb::parsing::select_parser::func::parse_select;

fn main() -> io::Result<()> {
    println!(
        "\tBienvenue dans {} / Version : 1.0 ",
        style("dirkdb").cyan()
    );
    println!("\n\tColaborateur : tomefydirk tony_mushah\n");
    loop {
        println!("{}",style("|DirkDB> ").bold().italic().red());
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().eq_ignore_ascii_case("QUIT"){
            return Ok(());
        }

        let b = parse_select(&buffer).expect("erreur illogique");
        println!("\n{:#?}\n", b.1.eval());
    }
}
