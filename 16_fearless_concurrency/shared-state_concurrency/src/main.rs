//Message passing is a fine way of handling concurrency, 
//but it’s not the only one. Another method would be for multiple threads to access the same shared data.

use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let m = Mutex::new(5); //mutex work as a locker fo data that can be accessed by threads

    {
        let mut num = m.lock().unwrap(); //This "lock" call will block the current thread 
                                                              //so it can’t do any work until it’s our turn to have the lock
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0)); //atomic, is safe to share across different threads
                                                    //leads to performance penalty
    let mut handles = vec![];

    for _ in 0..10 { //creating 10 threads that work woth counter var
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            //increasing num by 1 in each thread
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { //making sure each thread ends
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); //10
}