
fn main() {
    // Declare an immutable variable and initialize it with an i32 type 
    // This is a scalar type, has a fixed size and is stored on the stack
    let x = 20;
    // Declare another immutable variable and initilaize it with another stack variable
    // This copys the value stored in memory
    let y = x;

    // There is now transfer of ownership for scalar types, which means we can use them both
    println!("X is: {x} and y is: {y}");
}
