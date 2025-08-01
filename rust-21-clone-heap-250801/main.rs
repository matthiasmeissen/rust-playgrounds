fn main() {
    // Declare a immuntable variable called s1
    // It is a complex (pointer, length, capacity) pointer holding an address to some memory on the heap 
    // That stores the value of a string literal
    // The variable is the owner of that pointer
    let s1 = String::from("Hello");
    println!("S1: {}", s1);

    println!("--------");

    // Copy the data in memory (that s1 is pointing to) to a new location and store the pointer (and len, cap) in a variable
    // Note that s1 and the data in memory it is pointing to is still valid
    let s2 = s1.clone();
    println!("S1: {}", s1);
    println!("S2: {}", s2);

    println!("--------");

    // This is in contrast to the following line
    // Here we create a new variable and only transfer the ownership of the pointer from s2 to s3
    // This means s2 is no longer valid
    let s3 = s2;
    // This line will cause a compile error
    //println!("S2: {}", s2);
    println!("S3: {}", s3);
}