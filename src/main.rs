use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select id as p from employee";

    let b=parse_select(a).expect("erreur illogique");
    println!("{b:?}");
    println!("{:#?}",b.1.eval());


   
}
