use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select employee.nom from (select * from employee)";
    let b=parse_select(a).expect("erreur illogique");
    println!("{:#?}",b.1.eval());


   
}
