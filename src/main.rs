use dirkdb::parsing::select_parser::func::parse_select;


fn main()  {
    let a="select j.id from (select e.id from employee e where id=1) j where 1=1";
    let b=parse_select(a).expect("erreur illogique");
  
    println!("{:#?}",b.1.eval());


   
}
