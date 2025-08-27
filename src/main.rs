use dirkdb::{
    parsing::{select_parser::func::parse_select},
};
 fn test_select_queries() {
       

        println!("13°) {:?}\n", parse_select("SELECT sub.id, sub.total FROM (table) "));

    }

fn main() {
   test_select_queries();
}
