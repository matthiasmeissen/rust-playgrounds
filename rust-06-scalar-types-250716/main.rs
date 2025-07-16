
fn main() {
    // Integers come in signed and unsigned
    let int: u16 = 400;
    println!("This is an usigned 16 bit integer: {int}");
    let int2: i16 = -200;
    println!("This is a signed 16 bit integer: {int2}");

    // Floats are always signed and are available as 32 or 64 bit
    // The default is f64 since there is almost no difference in speed on mordern machines
    let float: f32 = -24.37829;
    println!("This is an signed 32 bit float: {float}");

    // When you do not specify the type on declaration it determines based on initialized value
    let automatic_float = 4.2;
    println!("This variable is an signed 64 bit float: {automatic_float}");

    // Booleans only have two states, one byte (why so much, might be that the smalles storage size in memory is always one byte)
    let bool = true;
    println!("This boolean is: {bool}");

    // Characters also just come in one format
    // How many bits does a char have in memory, there might be different options available for that
    // It has 4 bytes and represents a unicode scalar value, which has a lot of characters
    let char = 'R';
    println!("This character is: {char}");

    // To print a character in binary you need to cast it to an integer type
    println!("This character in binary is: {:b}", char as u32);
}
