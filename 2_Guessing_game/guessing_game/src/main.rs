use std::io; //bringing input/output library from std lib
use rand::Rng; //rng defines methods that random number generators implement
use std::cmp::Ordering; //enum for comparison

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //threated_rng gives rand generator seeded by os, range inclusive

    //println!("The secret number is: {secret_number}");

    loop { //infinite, quit by ^C or by typing "quit"
    println!("Please input your guess.");

    //let - creating variable, mut to define it as mutable (variables are unmut by default)
    let mut guess = String::new(); //string::new() returns new instance of str
    //:: is for pointing out association func of type written before ::

    io::stdin() // a type that represents a handle to the standard input for terminal
        .read_line(&mut guess) //getting input of user from terminal, storing it in var mentioned in brackets
        //& means its a reference
        .expect("Failed to read line"); //part of logical line of code (these lines could be written in one)
    //readline also return enum to know if func was executed successfully, basically means we work with compile errors on our own using.expect
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, //next loop step
    };
    //1 rust shadows previous value of var with new
    //2 trim() deletes whitespaces and newlines, parse is parsing :) (?)


    println!("You guessed: {guess}");//{} allow formated output like c# 

    //match is similar to c# switch (?)
    match guess.cmp(&secret_number) { //cmp = compare, returns variant of ordering
        Ordering::Less => println!("Too small!"), //arms
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
            println!("You win!");
            break; //loop exit
    }
}
}
}