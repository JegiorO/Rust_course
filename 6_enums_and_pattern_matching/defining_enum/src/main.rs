//Enums allow you to define a type by enumerating its possible variants.
//example of ip (which has types of v4 and v6)
enum IpAddrKind { //possible vars of var which cant be repeated in one dignity
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn route(ip_kind: IpAddrKind) {} //now can be passed to func as one type


struct IpAddr2 { //could use enum with struct like this
    kind: IpAddrKind,
    address: String,
}
 fn main2() {
    let home = IpAddr2 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr2 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
 }
//However, representing the same concept using 
//just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
enum IpAddr {
    V4(String),
    V6(String),
}
fn main3() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
} //That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.

//There’s another advantage to using an enum rather than a struct: 
//each variant can have different types and amounts of associated data. 
//Version four IP addresses will always have four numeric components that will have values 
//between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express 
//V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main4() {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
}

// this one has a wide variety of types embedded in its variants. if made with structs, it
// would have need 4 different structs, which cant be used in one func 
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, 
//we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

//The Option type is like enum but encodes the very common scenario 
//in which a value could be something or it could be nothing.
//The problem with null values is that if you try to use a null value as a not-null value, 
//you’ll get an error of some kind. 
//Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.
enum Option<T> {
    None,
    Some(T),
}
//<T> means that the Some variant of the Option enum can hold one piece of data of any type, 
//and each concrete type that gets used in place of T makes the overall Option<T> type a different type
fn main5() {
    let some_number = Some(5);
    let some_char = Some('e');
    //let absent_number: Option<i32> = None; //?????????????
}