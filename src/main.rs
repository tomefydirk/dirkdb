use dirkdb::parsing::select_parser::func::parse_select;

fn main() {
    let a = "select SQRT(4) p";

    let b = parse_select(a).expect("erreur illogique");
    println!("{b:?}");
    println!("{:#?}", b.1.eval());
}
