//you can specify that a particular bit of code be run whenever a value goes out of scope, 
//and the compiler will insert this code automatically. As a result, you don’t need to be careful 
//about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!
//You specify the code to run when a value goes out of scope by implementing the Drop trait

struct CustomSmartPointer {
    data: String,
}

//note that we’re not allowed to explicitly call drop,
//only std::mem::drop function
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    //Dropping CustomSmartPointer with data `other stuff`! printed
    //dropping CustomSmartPointer with data `my stuff`! printed
}