//To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle.

//a way without structures
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

//The area function is supposed to calculate the area of one rectangle, 
//but the function we wrote has two parameters, 
//and it’s not clear anywhere in our program that the parameters are related. 
//It would be more readable and more manageable to group width and height together.
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//programm using tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
//tuples don’t name their elements, 
//so we have to index into the parts of the tuple, making our calculation less obvious.

//programm with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //It’d be useful to be able to print an instance of Rectangle 
    //while we’re debugging our program and see the values for all its fields
    //println!("rect1 is {}", rect1); //but this doesnt work, structs don’t have a provided implementation of Display to use with println! and the {} placeholder.

    println!("rect1 is {:?}", rect1);
    //specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us 
    //to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
    //but that still doesnt work (`Rectangle` doesn't implement `Debug`)
    //To do that, we add the outer attribute #[derive(Debug)] just before the struct definition
    //{:#?} style makes it prettier (wrote in println!)

    dbg!(&rect1); //prints the file and line number of where that dbg! macro call occurs in your code 
    //along with the resultant value of that expression, and returns ownership of the value

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}