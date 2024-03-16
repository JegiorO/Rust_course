//Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. 
    //If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.
    //We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else. This is a closure that takes no parameters itself 
    //(if the closure had parameters, they would appear between the two vertical bars). The body of the closure calls self.most_stocked().

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use std::thread;
use std::time::Duration;
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );




    //CLOSURES features////////////////////////

    //adding type annotation
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    //these have same functionality
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x| { x + 1 }; //???????????????????????
    //let add_one_v4 = |x|  x + 1  ;


    //this wont work
    //let example_closure = |x:| x;
    //let s = example_closure(String::from("hello"));
    //let n = example_closure(5);
    //The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. 
    //Those types are then locked into the closure in example_closure, and we get a type error when we next try to use a different type with the same 
    //closure.

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    //here list is captured by closure
    borrows_mutably();
    println!("After calling closure: {:?}", list);


    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    //moving list to another thread using closure
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}