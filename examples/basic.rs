use yyid::*;

fn main() {
    let yyid = Yyid::nil();
    println!("\nUsing the Yyid::nil()");
    println!("[Display] [yyid:X]   {yyid:X}");
    println!("[Display] [Hyphen]   {}", yyid.as_hyphenated());
    println!("[Display] [Simple]   {}", yyid.as_simple());
    println!("[Display] [URN]      {}", yyid.as_urn());
    println!("[Display] [Braced]   {}", yyid.as_braced());
    println!("---------------------------------------------------------------------");
    println!("[Debug]   [yyid:?]   {yyid:?}");
    println!("[Debug]   [Hyphen]   {:?}", yyid.as_hyphenated());
    println!("[Debug]   [Simple]   {:?}", yyid.as_simple());
    println!("[Debug]   [URN]      {:?}", yyid.as_urn());
    println!("[Debug]   [Braced]   {:?}", yyid.as_braced());

    println!("\n=====================================================================");

    let yyid = Yyid::new();
    println!("\nUsing the Yyid::new()");
    println!("[Display] [yyid:X]   {yyid:X}");
    println!("[Display] [Hyphen]   {}", yyid.as_hyphenated());
    println!("[Display] [Simple]   {}", yyid.as_simple());
    println!("[Display] [URN]      {}", yyid.as_urn());
    println!("[Display] [Braced]   {}", yyid.as_braced());
    println!("---------------------------------------------------------------------");
    println!("[Debug]   [yyid:?]   {yyid:?}");
    println!("[Debug]   [Hyphen]   {:?}", yyid.as_hyphenated());
    println!("[Debug]   [Simple]   {:?}", yyid.as_simple());
    println!("[Debug]   [URN]      {:?}", yyid.as_urn());
    println!("[Debug]   [Braced]   {:?}", yyid.as_braced());
}
