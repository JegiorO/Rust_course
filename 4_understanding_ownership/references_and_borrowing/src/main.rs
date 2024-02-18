fn main() {
    // References allow us to work with ownership more convinient
    let s1 = String::from("hello");

    // &s1 is reference to the value of s1 but does not own it
    let len = calculate_length(&s1);
    
    // So, we havent dropped s1
    println!("The length of '{}' is {}.", s1, len);

    // example of immutable reference
    let s = String::from("hello");

    // doesnt work due to immutability of reference
    // change(&s);

    // Now we created a mutable string
    let mut s = String::from("hello");

    // and give mutable reference in params
    change(&mut s);

    // we cant have more than one mutable reference to one value
    // so, it code below doesnt work
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // scopes allow us to use some mutable reference
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // that example doesnt work because we cant have mutable reference
    // when we have immutable one
    let mut s = String::from("hello");

    // multiple immutable references are allowed
    // because no one can change them, they`re "read-only"
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM


    // We can create mutable ref in this exampe
    // because it is created after the last moment
    // immutable one was used
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // rust doesnt allow dangling ref
    //let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s //returning reference to the variable which will be shadowed after func executed:
// }      //rust wont allow this

// So, you should return value. not reference
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// This function calculate String length
// given as reference in params
// Returns only usize
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// it is supposed to change string but it doesnt work
// due to immutability of reference
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// This function change string given as mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}