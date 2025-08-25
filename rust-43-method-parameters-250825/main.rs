// Declare struct of type Rectangle
// that derives the Debug trait (the compiler will implement it)
#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Use the implementation block to add methods to the Rectangle type
impl Rectangle {
    // Each method must use self: &Self as the first argument 
    // Which is a reference to the element the mothod is called on 
    // You can also only write &self, or self in the parameters
    // Note the when we want to borrow the value, we need to use &self as a parameter
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // Create a second argument that is a reference to a Rectangle type
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    // Instantiate a Rectangle and store into variable
    let rect1 = Rectangle {
        width: 400,
        height: 200,
    };

    // Use the dubug format specifier to print the rect1 details
    // Worsk since we have implemented the debug trait on this type
    println!("rect1 is: {:?}", rect1);

    println!(
        "The area of rect1 is {}",
        rect1.area()
    );

    // Create new instance of Rectangle type
    let rect2 = Rectangle {
        width: 120,
        height: 80,
    };

    // Use the can hold method on rect1 and pass a reference to &rect2 as argument to it
    println!(
        "rect1 can hold rect2: {}",
        rect1.can_hold(&rect2)
    );
}