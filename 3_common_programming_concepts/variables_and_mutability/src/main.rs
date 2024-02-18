fn main() {
    let mut x = 5; //if we want to change variable we write mut when initilizing it
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //const is always immut, type must be annotated

    let x = 5;
    let x = x + 1; //shadows prev x var

    { //if its inner scope, var is re-declared just in this area
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //12
    }

    println!("The value of x is: {x}"); //6

    let spaces = "   ";
    let spaces = spaces.len(); //unlike mutability, shadowing allows type change
}