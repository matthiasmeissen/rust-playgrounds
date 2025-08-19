
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 20,
    }

    println!("The rect has a width of {rect1.width}cm and a height of {rect1.height}cm.");
}
