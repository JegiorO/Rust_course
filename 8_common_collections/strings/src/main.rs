//The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.0
// When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.

//Many of the same operations available with Vec<T> are available with String as well, 
//because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
fn main() {
    let mut s = String::new();

    //initaliztions
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    //A String can grow in size and its contents can change, just like the contents of a Vec<T>, 
    //if you push more data into it. In addition, you can conveniently use the + operator or the format! macro to concatenate String values.
    let mut s = String::from("foo");
    s.push_str("bar"); //push to the end

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //For more complicated string combining, we can instead use the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //rust doesnt allow string indexing, this gives error:
    // let s1 = String::from("hello");
    // let h = s1[0];

    for c in "Зд".chars() {
        println!("{c}");
    }
    //зд

    for b in "Зд".bytes() {
    println!("{b}");
//  208
//  151
//  208
//  180

}

}