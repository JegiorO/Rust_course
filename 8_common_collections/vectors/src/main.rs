//Rust’s standard library includes a number of very useful data structures called collections. 
//Most other data types represent one specific value, but collections can contain multiple values. 
//Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, 
//which means the amount of data does not need 
//to be known at compile time and can grow or shrink as the program runs.

//The first collection type we’ll look at is Vec<T>, also known as a vector. 
//Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 
//Vectors can only store values of the same type.

fn main() {
    //creating vector
    let v: Vec<i32> = Vec::new(); //Note that we added a type annotation here. 
    //Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.

    //More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store
    let v = vec![1, 2, 3]; //the vec! macro, which will create a new vector that holds the values you give it

    let mut v = Vec::new();
    v.push(5); //adding elements to vector
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; //reading via indexing, gives us reference
    let third: Option<&i32> = v.get(2); //reading via method get, gives us Option<&T>

    //following doesnt work cause first is immut while v itself is so
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    //This error is due to the way vectors work: because vectors put the values next to each other in memory, 
    //adding a new element onto the end of the vector might require allocating new memory and copying 
    //the old elements to the new space, if there isn’t enough room to put all the elements next to each other 
    //where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. 
    //The borrowing rules prevent programs from ending up in that situation.

    //Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //we have to use the * dereference operator to get to the value in i before we can use the += operator (Chapter 15)
    }

    //Vectors can only store values that are the same type. 
    //This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
    //so enums will help

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //Like any other struct, a vector is freed when it goes out of scope, as annotated in Listing 8-10.
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

}