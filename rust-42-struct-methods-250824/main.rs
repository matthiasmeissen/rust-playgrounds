// Declare struct of type Rectangle with two members
// Implement Debug Trait with derive (code that writes code for you)
#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To add functions to a struct type you need to create an implementation block for that type
impl Rectangle {
    // Inside that implementation block you can define your functions
    // The first argument of every function is &self (a reference to the instance the method is called on)
    // Note that &self is a shorthand for (self: &Self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Calculate aspect ratio of rectangle
    fn aspect(&self) -> f32 {
        // Since the width and height are both integers 
        // We need to cast them to float types before division
        (self.width) as f32 / (self.height) as f32
    }
}

fn main() {
    // Initialize a new Rectangle and store in variable
    let rect1 = Rectangle {
        width: 320,
        height: 140,
    };

    // Use debug format specifier in println!() macro to print the rectangle details in one line
    println!("Rect1 is: {:?}", rect1);

    // Pass a reference to rect1 in the calculate_area() function to get its area
    let area = calculate_area(&rect1);
    println!("The area of rect1 is: {area}");

    // To call a method on an instance of a struct you use dot notation including the name, paranthesis 
    // and any parameters you want to pass into it
    println!(
        "The area of rect1 is: {}",
        rect1.area()
    );

    println!(
        "The aspect ratio of rect1 is: {}",
        rect1.aspect()
    );
}

// This function has one parameter, reference to a Rectangle type and returns a unsigned 32bit integer
// This can be done smarter with methods, so the function is connected with the struct
fn calculate_area(r: &Rectangle) -> u32 {
    r.width * r.height
}