
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create instance of Rectangle and store in variable
    let rect1 = Rectangle {
        width: 20,
        height: 10,
    }
    
    // Use print to get details (a bit complicated and not scalable)
    println!("rect1 width: {} height: {}", rect1.width, rect1.height);


    let w = rect1.width;
    // You can print a simple type by just adding it to the println!() macro
    println!("The width of rect1 is {}", w);
    // Without the debug trait the following line will cause an error
    println!("rect1 is {}", rect1);
}
