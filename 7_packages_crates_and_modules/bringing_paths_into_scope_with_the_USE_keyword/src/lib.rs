//Having to write out the paths to call functions can feel inconvenient and repetitive.
//Fortunately, there’s a way to simplify this process: we can create a shortcut to a 
//path with the use keyword once, and then use the shorter name everywhere else in the scope.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //now no need to wtire full name everytime we use
//Paths brought into scope with use also check privacy, like any other paths.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//Note that use only creates the shortcut for the particular scope in which the use occurs.
mod customer {
    use crate::front_of_house::hosting; //so we need to add use in the module of customer as well
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

//Bringing the function’s parent module into scope with use means we have to specify the parent module 
//when calling the function. Specifying the parent module when calling the function makes 
//it clear that the function isn’t locally defined while still minimizing repetition of the full path.
//so dont write like this:
use crate::front_of_house::hosting::add_to_waitlist;

//There’s another solution to the problem of bringing two types of the same name into the same scope with use: 
//after the path, we can specify as and a new local name, or alias, for the type.

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     ()
// }

// fn function2() -> IoResult<()> {
//     ()
// }


//When we bring a name into scope with the use keyword, 
//the name available in the new scope is private. To enable the code that calls our code to refer to that name as 
//if it had been defined in that code’s scope, we can combine pub and use.

mod front_of_house2 {
    pub mod hosting2 {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}