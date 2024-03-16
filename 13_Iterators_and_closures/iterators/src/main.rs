//The iterator pattern allows you to perform some task on a sequence of items in turn. 
//An iterator is responsible for the logic of iterating over each item and 
//determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); //iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }

    //Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //.collect is collecting new iterator into collection
    assert_eq!(v2, vec![2, 3, 4]);
}
