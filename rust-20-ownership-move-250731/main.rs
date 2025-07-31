fn main() {
    // Declare a immutable variable and initialize with (complex) pointer to a value of type String, stored on heap memory
    // For the String data type the pointer holds three things: ptr, len, cap
    // The cariable s1 can be thought of as the owner of the pointer, when it goes out of scope it will drop it
    let s1 = String::from("Hello");
    println!("S1 is: {s1}");

    // Declare a immutable variable and assign the pointer to the location in memory s1 is holding
    // This moves the ownership from s1 to s2
    let s2 = s1;
    println!("S2 is also: {s2}");

    // When you try to compile this code you get an error since s1 is no longer valid
    // println!("{s1}");

    // When you really want to have both variables you can clone it
    // But this creates a copy, so you have two times the heap memory used
    let mut s3 = s2.clone();
    println!("S2 is still: {s2}");
    println!("S3 is: {s3}");

    // When you create a new String with a value and assign it to the existing complex pointer s3
    // It will drop the old data in memory and just use the new one
    s3  = String::from("People");
    println!("S3 is now: {s3}");
}