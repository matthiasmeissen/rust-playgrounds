// Declare struct of type Rectangle and implement Debug trait with derive (code hat writes code to implement it)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Use an implementation block to add methods to the Rectangle type
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // The following function also works
    // But Rectangle can be replaced with Self, which is an alias to the type the function is implemented for
    // This makes it more robust in case you would change the struct name

    // fn square(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size,
    //     }
    // }

    // This function does not use self as first parameter
    // It needs to be called with :: to work
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Create new instance of Rectangle type and store in variable
    let rect1 = Rectangle {
        width: 200,
        height: 120,
    };

    // Use debug format specifier to print rect1 details
    println!("rect1 is {:?}", rect1);

    // This calls the area() method in the struct
    println!(
        "The area of rect1 is: {}",
        rect1.area()
    );

    
    // This calls an associated function within the Rectangle namespace
    // The :: tells the compiler to look for any functions that are implemented for Rectangle with this name
    // It does not have an istance to work with, but its purpose is to create one
    // This is also how String::from() is implemented
    let rect2 = Rectangle::square(80);

    println!("rect2 is {:?}", rect2);
}