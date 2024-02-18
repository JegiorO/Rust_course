fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(2);

    main2();
}

fn another_function() { //can be defined anywhere in scope
    println!("Another function.");
}

fn another_function2(x: i32) { //with param
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) { //multiple params
    println!("The measurement is: {value}{unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. 

fn main2() {
    let y = {
        let x = 3;
        x + 1 //Expressions do not include ending semicolons
    };

    println!("The value of y is: {y}"); //4, cause x+1 is expression
}

fn five() -> i32 { //returns i32
    5
}
