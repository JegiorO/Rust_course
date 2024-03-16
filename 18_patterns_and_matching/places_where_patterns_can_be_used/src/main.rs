fn main() {
    // using pattern mathcing with 'if let' statement
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // determines what color to make your background
    // based on a series of checks for several conditions
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    // using vector as a stack
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // printing top values from stack
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // iterating in a vector
    let v = vec!['a', 'b', 'c'];
    // using a tuple as a pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // pattern mathing in let statement
    let (x, y, z) = (1, 2, 3);

    // pattern matching used in function params
    let point = (3, 5);
    print_coordinates(&point);

    // solving a problem of refutable pattern
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

// this function prints point coordinates
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}