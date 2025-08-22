// A tupel struct is define with the keyword struct and a name
// It the uses () and any number of comma separated types inside

// The i32 data type allows positive and negative numbers
struct Point (i32, i32);

// The u8 data type allows values in the range of 0-255 which is perfect for colors
struct Color(u8, u8, u8);

fn main() {
    // Create struct of type Point and assing values to it
    let point1 = Point(0, 4);

    // Use dot notation to access the values and print them
    println!("Point x is {} and point y is {}", point1.0, point1.1);

    let red = Color(255, 0, 0);
    println!("The color red has the values: {}, {}, {}", red.0, red.1, red.2);

    // To destructure a Tupel struct you need to add the name before the varaibles you want to place them in
    let Color(r, g, b) = red;
    println!("The color red has the values: {r}, {g}, {b}");
}