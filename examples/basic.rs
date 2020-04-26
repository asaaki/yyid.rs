use yyid::*;

fn main() {
    let yyid = YYID::new();
    println!("[Display] my yyid is: {}", yyid);
    println!("[Debug]   my yyid is: {:#?}", yyid);
}
