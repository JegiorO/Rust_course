//panic is for unrevocarable errors
fn main() {
    panic!("crash and burn"); //a way to implement panic

    let v = vec![1, 2, 3];
    v[99]; //rust will panic, no index 99 for our vector
}


//RESULT is for revoverable errors
//Result enum is defined as having two variants, Ok and Err
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
//func that returns result
fn main2() {
    let greeting_file_result = File::open("hello.txt");
}
//he return type of File::open is a Result<T, E>. 
//The generic parameter T has been filled in by the implementation of File::open with the type of the success value, 
//std::fs::File, which is a file handle. The type of E used in the error value is std::io::Error. 
//This return type means the call to File::open might succeed and return a file handle that we can read from or write to. 
//The function call also might fail: for example, the file might not exist, or we might not have permission to access the file. 
//The File::open function needs to have a way to tell us whether it succeeded or failed and at the same time 
//give us either the file handle or error information. 
//This information is exactly what the Result enum conveys.

//handling errors with match
fn main3() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
} //will panic! no matter why File::open failed

//different panics with multiple matching
use std::io::ErrorKind;
fn main4() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

//The Result<T, E> type has many helper methods defined on it to do various, more specific tasks. 
//The unwrap method is a shortcut method implemented just like the match expression we wrote in Listing 9-4. 
//If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
//If the Result is the Err variant, unwrap will call the panic! macro for us.

fn main5() {
    let greeting_file = File::open("hello.txt").unwrap();
}

//Similarly, the expect method lets us also choose the panic! error message. 
fn main6() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
//We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The error message 
//used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses.


//When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return 
//the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code

use std::io::{self, Read};

// If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), //returning error itself
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
//same functionality but shorter using ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
//The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined 
//to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned 
//from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function 
//as if we had used the return 
//keyword so the error value gets propagated to the calling code.
//note that ? is simplier than match but doext allow us to have custom errors

//the shortest var of this func
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

//The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.

// fn main7() {
//     let greeting_file = File::open("hello.txt")?;
// }
//the ? operator follows the Result value returned by File::open, but this main function has the return type of (), not Result.
//mb cause otherwise it will try to return error from main which is impossible..

//? can be used with Option<T> values as well. As with using ? on Result, you can only use ? 
//on Option in a function that returns an Option. The behavior of the ? operator when called on an Option<T> 
//is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early 
//from the function at that point. If the value is Some, the value inside the 
//Some is the resulting value of the expression and the function continues.

//Note that you can use the ? operator on a Result in a function that returns Result, 
//and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.

//Luckily, main can also return a Result<(), E>. following has the code from prev func but we’ve changed the return type of main to be Result<(), 
//Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile:

use std::error::Error;
fn main100() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}