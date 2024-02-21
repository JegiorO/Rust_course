use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub mod guess;
fn main() {
    println!("Guess the number!");
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        // var to store guess
        let mut guess = String::new();
        // read input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // create new guess instance
        let guess = guess::Guess::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });

        // Check user guess
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// This function shows how we can hardcode IP
fn hardcoded_ip() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        // using an expect to unwwrap a Result<T, E>
        .expect("Hardcoded IP address should be valid");
}