// Define new struct called Rectangle
// It has two members of type u32 and one of type String
struct Rectangle {
    width: u32,
    height: u32,
    unit: String,
}

fn main() {
    // Create new instance of Rect and store pointer to it in variable
    let rect1 = Rectangle {
        width: 200,
        height: 140,
        unit: String::from("cm"),
    };

    // Pass reference to Rectangle into function to print details
    print_rect_details(&rect1);

    let rect2 = create_cm_rect(400, 200);
    print_rect_details(&rect2);

    // This uses struct update syntax
    // It uses all values from rect2 and only adjusts those that are mentioned
    let rect3 = Rectangle {
        unit: String::from("m"),
        ..rect2
    };
    print_rect_details(&rect3);
}

fn print_rect_details(r: &Rectangle) {
    let w = r.width;
    let h = r.height;
    let area = w * h;
    // Create a reference to the String inside the function to prevent movement of ownership
    let u = &r.unit;
    println!("A rectangle with a width of {w}{u} and a height of {h}{u} has an area of {area}{u}.");
}

// This function takes width and height as argument and returns a Rectangle with unit set
fn create_cm_rect(width: u32, height: u32) -> Rectangle {
    // Rectangle {
    //     width: width,
    //     height: height,
    //     unit: String::from("cm"),
    // }

    // When key and value have the same name, you can simplify by only writing the name
    // This is called field init shorthand

    Rectangle {
        width,
        height,
        unit: String::from("cm"),
    }
}