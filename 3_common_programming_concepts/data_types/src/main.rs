fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); //if :u32 isnt added it gives error

    //SCALAR TYPES
    let x : i8 = 1; //signed int, 8 bytes (range is 8-128)
    let x : u8 = 1; //unsigned int
    //overflow is either error (debug) or is giving unexpected number

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.0 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1 (truncates toward zero to the nearest integer)
    // remainder
    let remainder = 43 % 5;

    //BOOLEAN
    let t = true;
    let f: bool = false; // with explicit type annotation

    //CHAR
    let c = 'z'; //single quotes only
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //COMPOUND TYPES
    //TUPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1); //variety of types into one compound type, fixed length
    let (x, y, z) = tup; //to get value from tup
    let five_hundred = tup.0; //or directly
    println!("The value of y is: {y}"); //6.4
    //ARRAY
    let a = [1, 2, 3, 4, 5]; //fixed length, one type
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //type then length
    let a = [3; 5]; //to initilize 5 elements =3
    let first = a[0]; //access to el
    let element = a[10]; //panic

}
