use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path) //opens the file on path
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    //will print poem.txt
}
