
fn main() {
    // Creating a closure like this and storing it in as a variable wont work
    let add_one = |x| x + 1;

    // Until we use the closure with some type, here the compiler can infere the type and link it to the initial closure definition
    println!("20 + 1 is: {}", add_one(20));

    // Having use it once with one type will not allow to use it again with another
    //println!("20.0 + 1.0 is: {}", add_one(20.0));

    // A closre can be declared with explicit types as well
    let add_two = |x: i32| -> i32 {x + 2};

    println!("20 + 2 is: {}", add_two(20));
}
