
fn main() {
    // Declare a immuntable variable called s1
    // It is a complex (pointer, length, capacity) pointer holding an address to some memory on the heap 
    // That stores the value of a string literal
    let s1 = String::from("Hello");
    println!("The value of s1 is: {}", s1);
}
