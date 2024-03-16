use std::ops::Add;
use std::fmt;

pub trait Iterator {
    // type placeholder
    type Item;
    // get next item
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter {
    count: u32,
}

impl Counter {
    // create counter
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; //stating type is u32
    // get new item in counter
    fn next(&mut self) -> Option<Self::Item> {

        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
// overriding an add operator for point
impl Add for Point {
    // default right hand side type is Self=Point
    type Output = Point;
    // adds coordinate
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);
// now we use Meters instead of default type as we want to add different types
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // add meters to millimiters
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// print something as
// **********
// *        *
// * (1, 3) *
// *        *
// **********
// only for types that impement Display trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}