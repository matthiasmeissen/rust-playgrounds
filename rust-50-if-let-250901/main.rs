// Define enum of type Shape with two variants
enum Shape {
    Rectangle,
    Circle,
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Create new instance of Shape with variant Rectangle
    let shape1 = Shape::Rectangle;
    // Pass variable into function to get details
    get_shape_details(shape1);

    // Create new variable of type Option<u32> with passing the same type into the function
    let valid_num = add_one(Some(20));

    // Use the match control flow to get the value stored in Option<u32> and print it
    match valid_num {
        Some(i) => println!("The value of new_num is {i}"),
        None => println!("The value of new_num is None"),
    };

    // Create a variable that has a None variant of the Option<u32> type
    let invalid_num = add_one(None);

    // This is a more compact way to define what should happen for one specific variant of an enum and ignore all others
    let some_number = Some(40);
    if let Some(num) = some_number {
        println!("The value of some_number is {num}");
    }

    // This is a good way to access avlues that are associated with a variant of an enum
    let local = IpAddrKind::V4(127, 0, 0, 1);
    if let IpAddrKind::V4(a, b, c, d) = local {
        println!("The IP address is: {a}, {b}, {c}, {d}");
    }
}

// This function takes a Shape enum type as input
// It uses match to detect which variant is passed in
// And then prints some details
fn get_shape_details(s: Shape) {
    match s {
        Shape::Rectangle => println!("It is a rectangle."),
        Shape::Circle => println!("It is a circle"),
    }
}

// This function takes on Option<u32> as argument and also returns an Option<32>
fn add_one(n: Option<u32>) -> Option<u32> {
    // Use the match control flow to dfine how different cases are handled
    match n {
        // Note that we do not write Option<u32>::None but just use None to define the arm
        // This is since the Option<T> enum has been brought into scope automatically
        // So we do not need to add the namespace as we do with our own enums
        None => {
            println!("No value found");
            None
        },
        // The same applies to all other arms we define
        Some(i) => {
            println!("The initial value was {i} we add one to it.");
            Some(i + 1)
        }
    }
}