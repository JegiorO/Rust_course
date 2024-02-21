//match allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 
//Patterns can be made up of literal values, variable names, wildcards, and many other things

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { //This seems very similar to a conditional expression used with if, but there’s a big difference:
        // with if, the condition needs to evaluate to a Boolean value, but here it can be any type.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } //match arm
        Coin::Nickel => 5, //the => operator separates the pattern and the code to run
        Coin::Dime => 10, //match arm
        Coin::Quarter => 25,
    }
}

//Matching with Option<T>
//if there’s a value inside, adds 1 to that value. 
//If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

//Matches Are Exhaustive
//There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities. 
//Consider this version of our plus_one function, which has a bug and won’t compile:
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// } //We didn’t handle the None case, so this code will cause a bug

//Using enums, we can also take special actions for a few particular values, 
//but for all other values take one default action. 
//Imagine we’re implementing a game where, if you roll a 3 on a dice roll, 
//your player doesn’t move, but instead gets a new fancy hat. If you roll a 7, your player loses a fancy hat. 
//For all other values, your player moves that number of spaces on the game board.
fn main2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //The code that runs for the other arm 
        //uses the variable by passing it to the move_player function.
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

//Rust also has a pattern we can use when we want a catch-all 
//but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value. 
//This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {}
}
