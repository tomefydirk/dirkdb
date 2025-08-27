use dirkdb::parsing::{logic_parser::func::parse_logical, select_parser::func::parse_select};
 fn test_select_queries() {
       

        println!("13°) {:?}\n", parse_select("SELECT sub.id, sub.total+1 FROM (table) "));

    }

fn test_into_string(){
    let a=parse_logical("field+1+Now(1)*3").expect("erreur désole").1;
    println!("{}",*a);
}
fn main() {
   test_select_queries();
   test_into_string();
}
