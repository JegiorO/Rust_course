// this struct is responsible for validating a guess
pub struct Guess {
    value: i32,
}

impl Guess {
    // creaste new guess
    pub fn new(value: i32) -> Guess {
        // panic if calue is not in [1, 100]
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        // return guess if all is OK
        Guess { value }
    }
    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}