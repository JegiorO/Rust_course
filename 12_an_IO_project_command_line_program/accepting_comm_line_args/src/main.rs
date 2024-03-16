use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //returns an iterator of the command line arguments passed to minigrep
    //iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, 
    //such as a vector, that contains all the elements the iterator produces.
    //dbg!(args);
    //[src/main.rs:5] args = [
    //"target/debug/minigrep",

    //Saving the Argument Values in Variables
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

//Notice that the first value in the vector is "target/debug/minigrep", which is the name of our binary. 
//This matches the behavior of the arguments list in C, letting programs use the name by which they were invoked in their execution.