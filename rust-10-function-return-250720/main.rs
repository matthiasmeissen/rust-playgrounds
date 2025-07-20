// There is a distinction between statements and expressions

// Statements 
// Instructions that perform some action, but do not return a value.
// let x = 4;

// Expressions
// Things that evaluate to a value
// x + 4

fn main() {
    let x = 10;
    let y = plus_one(x);
    println!("x was {x}, now y is {y}");
    println!("x times y is {0}", multiply(x, y));
}

// You declare a function that returns a value with -> syntax
// The value returned is specified by its type
// You can use a function without writing return 
// Then the function implicitly returns the last expression
// You just have to be sure to not use a semicolon at the end for it
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn multiply(x: i32, y: i32) -> i32 {
    // While you could write the function like this
    // let result = x * y;
    // return result;

    // Writing it like that is way more compact
    x * y
}