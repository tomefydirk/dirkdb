use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select 1+1 p , sqrt(9) where 1=1 ";
    let b=parse_select(a).expect("erreur illogique");
    println!("{:#?}",b.1.eval());


   
}
