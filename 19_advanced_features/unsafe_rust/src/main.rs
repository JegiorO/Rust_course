use std::slice;

unsafe fn dangerous() {} //unsafe func
static HELLO_WORLD: &str = "Hello, world!"; //global vars
static mut COUNTER: u32 = 0;

// example of unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; //raw pointer defined
    let r2 = &mut num as *mut i32; //mutable one
    //wouldnt comiple with simple pointers

    let address = 0x012345usize; //raw pointer to a random memory address
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);//dereferencing raw pointers with a unsafe block
        println!("r2 is: {}", *r2);
    }
    unsafe {
        dangerous(); //cannot be called without unsafe block
    }

    // creating safe abstraction above unsafe code
    let mut vector = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = split_at_mut(&mut vector, 3);

    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);

    // calling an extern function is unsafe
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // use of global var
    println!("name is: {}", HELLO_WORLD);

    // changing static var
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// takes one slice and makes it two by splitting the slice at the index given as an argument
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // unsafe block wrapped by safe function
    unsafe {
        (
            // takes a raw pointer and a length and it creates a slice
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" { //taking func from another language
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe { //static vars can be changed only via unsafe code
        COUNTER += inc;
    }
}