use yyid::*;

fn main() {
    println!("Using the Yyid::nil()");
    let yyid = Yyid::nil();
    println!("[Display][Hyphen] {}", yyid);
    println!("[Display][Simple] {}", yyid.as_simple());
    println!("[Display][URN]    {}", yyid.as_urn());
    println!("[Debug][Hyphen]   {:?}", yyid);
    println!("[Debug]Simple]    {:?}", yyid.as_simple());
    println!("[Debug][URN]      {:?}", yyid.as_urn());

    println!("Using the Yyid::new()");
    let yyid = Yyid::new();
    println!("[Display][Hyphen] {}", yyid);
    println!("[Display][Simple] {}", yyid.as_simple());
    println!("[Display][URN]    {}", yyid.as_urn());
    println!("[Debug][Hyphen]   {:?}", yyid);
    println!("[Debug]Simple]    {:?}", yyid.as_simple());
    println!("[Debug][URN]      {:?}", yyid.as_urn());
}
