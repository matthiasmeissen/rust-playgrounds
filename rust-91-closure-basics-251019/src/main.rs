fn main() {
    // This defines a closure
    // It is like a function, but can be stored in a variable
    // In this version the typs is inferred by its first use.
    let plus_one = |x| x + 1;

    let result = plus_one(5);
    println!("5 + 1 is: {result}");

    // This means you can not use it with any other type again
    // let result2 = plus_one(5.0);

    // This closure has an explicit type annotation
    let plus_two = |x: f32| -> f32 {x + 2.0};
    println!("20.0 + 2.0 is: {:.1}", plus_two(20.0));

    // For comparison, this is a function that does a similar thing
    println!("40.0 + 1.0 is: {:.1}", add_one(20.0));
}

fn add_one(x: f32) -> f32 {
    x + 1.0
}