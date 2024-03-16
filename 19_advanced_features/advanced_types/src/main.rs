use std::cmp::Ordering;
use std::io;
fn main() {
    type Kilometers = i32; //synonym, another name for existing type

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>; // use synonymus to make type names shorter and more convinient

    let f: Thunk = Box::new(|| println!("hi")); //very long name, we pack it into Thunk and use easily
    fn takes_long_type(f: Thunk) {} //passed easily

    print!("forever ");
    loop { //type "never" is never returned, used in loops (!)
        print!("and ever ");
        break;
    }

    let answer = do_twice(add_one, 5); //func in params

    println!("The answer is: {}", answer);

    // can use closures and functions in map method
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }
    // can use these initializer functions as function pointers that implement the closure traits
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// if we dont know T size at the moment of compilation
// we use ?Sized, and then use &T
fn generic<T: ?Sized>(t: &T) {}

fn add_one(x: i32) -> i32 {
    x + 1
}
// takes function in params and do it twice
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// returns closures using pointers
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}