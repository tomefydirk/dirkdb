use dirkdb::parsing::select_parser::func::parse_select;

fn main() {
    let a = "select sqrt(p.id) from (select id from employee) p";

    let b = parse_select(a).expect("erreur illogique");
    println!("{b:?}");
    println!("{:#?}", b.1.eval());
}
