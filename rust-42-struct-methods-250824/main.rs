
// Declare struct of type Rectangle with two members
// Implement Debug Trait with derive (code that writes code for you)
#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Initialize a new Rectangle and store in variable
    let rect1 = Rectangle {
        width: 200,
        height: 100,
    };

    // Use debug format specifier in println!() macro to print the rectangle details in one line
    println!("Rect1 is: {:?}", rect1);
}
