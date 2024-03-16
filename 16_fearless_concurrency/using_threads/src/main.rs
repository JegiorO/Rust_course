use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| { //func to create new thread
        for i in 1..10 { //closure contains code that will be executed in new thread
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); //makes thread stop allowing other threads to execute
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //if we want to make sure all threads wew executed, we call join
    let handle = thread::spawn(|| { //saving JoinHandle of thread in var
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();//like this
    //we wait until handle is done and then call end of main


    //if we want to pass values, move is used in closure
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { //closure becomes the owner of values used by it
        println!("Here's a vector: {:?}", v);            //notice that v is no longer available in main thread
    });
    handle.join().unwrap();
}