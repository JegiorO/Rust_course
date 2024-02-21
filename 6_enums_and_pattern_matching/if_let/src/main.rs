//The if let syntax lets you combine if and let into 
//a less verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), //A match that only cares about executing code when the value is Some
    }
    //To satisfy the match expression, we have to add _ => () 
    //after processing just one variant, which is annoying boilerplate code to add.
    //Instead, we could write this in a shorter way using if let.

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}