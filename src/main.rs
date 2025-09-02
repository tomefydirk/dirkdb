use dirkdb::parsing::select_parser::func::parse_select;

fn main() {
    let a = "select sqrt(1)";

    let b = parse_select(a).expect("erreur illogique");
    println!("{b:?}");
    println!("{:#?}", b.1.eval());
}
