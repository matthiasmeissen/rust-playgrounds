// An Enum (Enumeration) is a collection of variants that belong together
// We enumerate all possible variants

// This enum create a custom data type called IpAddrKind
// It has the variants V4 and V6
// The V4 consists of four unsigned 8bit integers (0-255)
// The V6 is one String
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Create new instances of IpAddrKind enum with the variant V4 and V6
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // Use the println!() macro with the debug format specifier to print the variables
    println!(
        "The IP Address of home is: {:?} \nThe IP Address of loopback is: {:?}"
        , home, loopback
    );
}