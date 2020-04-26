use yyid::*;

fn main() {
    println!("Using the Yyid::nil()");
    let yyid = Yyid::nil();
    println!("[Display][Hyphen] {}", yyid);
    println!("[Display][Simple] {}", yyid.to_simple_ref());
    println!("[Display][URN]    {}", yyid.to_urn_ref());
    println!("[Debug][Hyphen]   {:?}", yyid);
    println!("[Debug]Simple]    {:?}", yyid.to_simple_ref());
    println!("[Debug][URN]      {:?}", yyid.to_urn_ref());
}
