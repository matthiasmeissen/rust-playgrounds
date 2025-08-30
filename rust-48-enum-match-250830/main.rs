// Define an enum of type Coin with four variants
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Define an enum of type Shape with two variant, each of which has different values stored inside
enum Shape {
    Rectangle(u32, u32),
    Circle(u32),
}

fn main() {
    // Create new instance of Coin and print its value using a function
    let coin1 = Coin::Penny;
    println!("The value in cents of coin1 is {}", value_in_cents(coin1));

    // Create a new instance of Shape and print the area of it using a function
    let shape1 = Shape::Rectangle(40, 20);
    println!("The area of shape1 is {}", get_area(shape1));
}

// This function takes a variable of type Coin as input and returns a 8bit integer
fn value_in_cents(coin: Coin) -> u8 {
    // The match control flow loops through the input until it finds a valid part
    // This condition does not evaluate to true or false, but to one specific type
    match coin {
        // With the Enum::Variant pattern you select which one you target
        // And with => you say what it should return
        // Simple expressions so not need curlce brackes, but a comma to separate
        Coin::Penny => {
            println!("This is some coin");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// This function takes a type Shape as parameter and returns a 32bit float
fn get_area(shape: Shape) -> f32 {
    match shape {
        // It also includes a typel after the pattern that matches the type
        // This is a way to get values out of an enum type
        // Since you need to specify every possible situation
        Shape::Rectangle(w, h) => (w * h) as f32,
        Shape::Circle(r) => (r * r) as f32 * 3.14,
    }
}