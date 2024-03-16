//Implementing the Deref trait allows you to customize the behavior of the 
//dereference operator * (not to be confused with the multiplication or glob operator).
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { //defining our realization for derefferencing
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); //When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
                        //*(y.deref())
}