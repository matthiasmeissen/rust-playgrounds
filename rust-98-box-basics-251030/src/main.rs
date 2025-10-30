use crate::List::{Cons, Nil};

fn main() {
    // A Box is a pointer to some value on the heap
    // So b contains a pointer to an i32 type and the value 5 located on the heap
    let b = Box::new(4);
    println!("b is: {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List is: {:?}", list);
}

// This is a recursive data structure, since it contains an instance of itself
// To let the compiler know how much space it needs to allocate we place the list in a Box
// By doing that Box becomes a knows sice, which is a pointer to the heap where the next List is placed
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}