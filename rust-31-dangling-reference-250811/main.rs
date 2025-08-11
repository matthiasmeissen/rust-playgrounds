fn main() {
    //let my_string = ref_inside();

    let my_string = allocate();
    println!("The value from the function is: {my_string}");
}

// This function is not valid
// Since it return a reference to a String
// But the String only exists during the lifetime of the function
// Handing over access to a String that is no longer existing would create a dangling pointer
// The compiler detects and throws an error

//fn ref_inside() -> &String {
//    let s = String::from("Hello");
//    &s
//}

fn allocate() -> String {
    let s = String::from("Hello");
    s
}