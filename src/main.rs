use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select e.id from employee e";
    let b=parse_select(a).expect("erreur illogique");
    println!("{:#?}",b.1.eval());


   
}
