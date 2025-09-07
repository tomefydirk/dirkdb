use std::io;

use dirkdb::cli::{introduction, request_reader};

fn main() -> io::Result<()> {
    introduction();
    request_reader()
}
/*
    todo!:
    {
    changer les erreurs &str par des string
    séréalisation
    }
*/
