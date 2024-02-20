//methods are similar to fnctns, but unlike functions, methods are defined within the context of a struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //implementation block where we can define methods for struct
    //Everything within this impl block will be associated with the Rectangle type
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool { //same name for method as for field, rust will know we call method cause we use parentheses
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // another example of method
        self.width > other.width && self.height > other.height
    }
}

//1 Each struct is allowed to have multiple impl blocks
//2 We can define associated functions that don’t have self as their first parameter 
//(and thus are not methods) because they don’t need an instance of the type to work with.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
//To call this associated function, 
//we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}