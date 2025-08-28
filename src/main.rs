use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select p from (select sqrt(id) p from users)";
    let b=parse_select(a).expect("erreur illogique");
    println!("{:?}",b.1.eval_with_from());
}
