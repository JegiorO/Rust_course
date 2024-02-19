fn main() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
    // A slice is a kind of reference, so it does not have ownership.
    //A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];


    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; //equal to prev line

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];//equal to prev line


    //fn first_word(s: &String) -> &str {
    //fn first_word(s: &str) -> &str { is better



    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

        //slice of array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes(); //Because we need to go through the String element by element and check whether a value is a space, 
    //we’ll convert our String to an array of bytes using the as_bytes method.

    for (i, &item) in bytes.iter().enumerate() { //The first element of the tuple returned from enumerate is the index, 
        //and the second element is a reference to the element
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str { //Now when we call first_word, we get back a single value that is tied to the underlying data. 
    //The value is made up of a reference to the starting point of the slice and the number of elements in the slice.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// fn main2() { //wont work with functions mentioned!!
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }

// fn main3() { //is better now cause an error will occure
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {}", word);
// }