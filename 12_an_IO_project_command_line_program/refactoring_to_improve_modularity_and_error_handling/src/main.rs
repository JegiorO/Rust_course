use std::env;
use std::process;
use refactoring_to_improve_modularity_and_error_handling::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    //Using unwrap_or_else allows us to define some custom, non-panic! error handling
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); //standart func for printing errors
        process::exit(1);
    });

    //cool error handling
    if let Err(e) = refactoring_to_improve_modularity_and_error_handling::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    
}
//separating code in funcs for atomicity
