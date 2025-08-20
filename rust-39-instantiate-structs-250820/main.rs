
// Define new struct called Rectangle
// It ha stwo members of type u32
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create new instance of Rect and store pointer to it in variable
    let rect1 = Rectangle {
        width: 200,
        height: 140,
    }

    // Pass reference to Rectangle into function to print details
    print_rect_details(&rect1);
}

fn print_rect_details(r: &Rectangle) {
    let w = r.width;
    let h = r.height;
    let area = w * h;
    println!("A rectangle with a width of {w} and a height of {h} has an area of {area}.");
}
