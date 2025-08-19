// Define a struct by using the struct keyword and a name
// The add names and types into it
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create a new instance of a struct with specific values
    let rect1 = Rectangle {
        width: 40,
        height: 20,
    };

    // Print details about the rect
    println!("-------- \nThe rect has a width of {} cm and a height of {} cm and an area of {} square cm.", rect1.width, rect1.height, rect1.width * rect1.height);

    // Create a mutable instance of type Rectangle
    let mut rect2 = Rectangle {
        width: 80,
        height: 140,
    };

    // Pass a reference to the rect into the function
    // This keeps the onwership at rect2 so we can use it again
    get_rect_details(&rect2);

    // Use the dot notation to change one value of rect2
    rect2.width = 200;

    get_rect_details(&rect2);
}

// This function takes a reference to a Rectangle type and prnts some details
fn get_rect_details(r: &Rectangle) {
    let w = r.width;
    let h = r.height;
    let area = w * h;
    println!("--------\n The rect has a width of {w} cm and a height of {h} cm and an area of {area} square cm.");
}