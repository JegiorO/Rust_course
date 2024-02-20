fn main() {
    let mut user1 = User { //the entire instance must be mutable; 
        //Rust doesn’t allow us to mark only certain fields as mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com"); //To get a specific value from a struct, we use dot

    // let user2 = User { 
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user3 = User { //same as written before but less code and same result
        email: String::from("another@example.com"),
        ..user1
    };

    struct Color(i32, i32, i32); //Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //Note that the black and origin values are different types because they’re instances
    //of different tuple structs. Each struct you define is its own type, 
    //even though the fields within the struct might have the same types. 
    //For example, a function that takes a parameter of type Color cannot take a Point as an argument, 
    //even though both types are made up of three i32 values

    struct AlwaysEqual; //unit-like struct, no fields
}

struct User { //defining struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User { //function that returns a User instance with the given email and username
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Because the parameter names and the struct field names are exactly the same in Listing 5-4, 
//we can use the field init shorthand syntax to rewrite build_user so it behaves 
//exactly the same but doesn’t have the repetition of username and email
fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
