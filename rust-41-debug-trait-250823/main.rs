// You can implement the debug trait for a struct
// The derive lets you tell the compiler to write the necessary code for you
// With the debug trait available on a type you can use 
// Debug {:?} format specifier 
// Pretty Debug {:#?} format specifiers 
// In a println!() they will format the details in different ways

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create instance of Rectangle and store in variable
    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };
    
    // Use print to get details (a bit complicated and not scalable)
    println!("rect1 width: {} height: {}", rect1.width, rect1.height);


    let w = rect1.width;
    // You can print a simple type by just adding it to the println!() macro
    println!("The width of rect1 is {}", w);
    // You can also use the {:?} or {:#?} format specifiers on built in types
    // but it won't look any different than without it
    println!("The width of rect1 is {:?}", w);
    println!("The width of rect1 is {:#?}", w);

    // This uses the debug trait we have added to the Rectangle type to show the details
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}