fn main() {
    // Unused variables should be prefixed with an underscore 
    // This helps the compiler so it won't show a warning
    let _x = 20;

    // To add a type annotation you insert a : after the name and then set the type
    let y: i32 = 40;
    println!("y is: {y}");

    // Integer types (which are part of the scalar types) are available as 
    // u: usigned (only positive)
    // i: signed (postive and negative)

    // In addition they can have different length in bit
    // 8, 16, 32, 64, 128
    // The default int is i32

    // Create an signed 8 bit variable, initialize with 100 (within 0-255 range)
    let mut num_unsigned_8: u8 = 100;
    println!("Num Signed 8 is: {num_unsigned_8}");

    // Add to it so that it exceeds range
    // num_unsigned_8 += 200;
    // This will overflow the variable and the program will panic in debug mode
    // It won't do so in release mode, where the number will wrap instead

    num_unsigned_8 += 140;
    println!("Num Signed 8 is now: {num_unsigned_8}");

    let num_signed_16: i16 = 2000;
    println!("Num Signed 16 is: {num_signed_16}");

    // Declaring an 32bit unsigned number and initialize it with a hex value
    // In order to specify a number as hexadecimal you need to prefix it with 0x
    let hex: u32 = 0x4a8d5b;
    println!("The number specified in hexadecimal is: {hex}");

    // Declaring a 8 bit unsigned number and assigning it using binary
    // In order to specify a number as binary you need to prefix it with 0b
    // The underscore is not necessary but allowed and helps to read the number
    let byte: u8 = 0b0000_1111;
    println!("The number specified in binary is: {byte}");

    // To print a number in a specific format you can use formatting specifiers
    println!("The number in hexadecimal is: {:X}", byte);
    println!("The number in binary is: {:b}", byte);
}