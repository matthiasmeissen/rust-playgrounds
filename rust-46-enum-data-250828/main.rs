// An enums group together differnt values 
// It is a way to say that a value is one of a possible set of values
#[derive (Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The Shape enum has different shape variants, which refer to structs
#[derive (Debug)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

// The Rectangle struct is composed of a width and height attribute
#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The implementation block for the Rectangle includes 
// An associated function called new, which acts as a constructor to create a Rectangle type
// An area method that can be used on an Rectangle instance to calculate its area
impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        Self{
            width: w,
            height: h,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive (Debug)]
struct Circle {
    radius: u32,
}

impl Circle {
    fn new(r: u32) -> Self {
        Self {
            radius: r
        }
    }
}

fn main() {
    // Create new instances of IpAddrKind enum and initialize with value
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // Print variables with debug format specifier to be used on type that has implmented the Debug trait
    println!("Home Ip is: {:?}", home);
    println!("Loopback Ip is: {:?}", loopback);

    let shape1 = Shape::Rectangle(Rectangle::new(20, 40));
    println!("Shape1 is: {:?}", shape1);

    let shape2 = Shape::Circle(Circle::new(20));
    println!("Shape2 is: {:?}", shape2);

    // Pass shape enum is a function to print its details
    print_shape_details(shape1);
}

fn print_shape_details(s: Shape) {
    println!("Shape is {:?}", s);
}