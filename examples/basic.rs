use yyid::*;

fn main() {
    let yyid = Yyid::new();
    println!("[Display] my yyid is: {}", yyid);
    println!("[Debug]   my yyid is: {:#?}", yyid);
}
