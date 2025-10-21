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


    // There are three types of ownership for closures

    // Immutable Borrow
    let x = 4;
    println!("Immutable:    Before Closure:     {x}");
    let borrow = || println!("Immutable:    Inside Closure:     {x}");
    println!("Immutable:    Before Call:        {x}");
    borrow();
    println!("Immutable:    After Call:         {x}");

    // Mutable Borrow - The closure variable must be declared as mutable
    let mut y = 4;
    println!("Mutable:      Before Closure:     {y}");
    let mut mutable = || y += 4;
    mutable();
    println!("Mutable:      After Closure:      {y}");

    // Take Ownership - The move keyword forces the closure to take ownerhsip of every value passed into
    let z = 4;

    std::thread::spawn(move || println!("Ownership:    Inside Thread:      {z}")).join().unwrap();
}