fn main() {
    // This creates a pointer to some memory on the heap that contains a string literal
    let mut my_string = String::from("Hello people");
    println!("{}", my_string);

    // Pass a reference to the pointer to the String
    print_str(&my_string);
    // Create a string literal, this is hardcoded into the binary and lives in read only memory 
    // The variable is a pointer to that memory and lives on the stack
    let my_other_string = "String Literal";
    print_str(my_other_string);

    // You can add elements to the String type on the heap
    // But not to the string literal, it lives on the stack and is read only
    my_string.push_str(" on this planet.");

    print_str(&my_string);

    // Create an empty String
    let s = String::new();
    print_str(&s);

    // You can create a String from a &str with the .to_string() method
    let data = "String from data.";
    let s = data.to_string();
    print_str(&s);

    // This can also be done directly on the string literal
    let s = "String from inline data.".to_string();
    print_str(&s);

    // This is an associated function that the String struct has implemented
    let mut s = String::from("Hello people");
    // A String is a special kind of collection, so as with other collections
    // You can call the .push_str() method to append a string literal to the mutable variable
    s.push_str(" on this planet");
    // You can also call the .push() method to append a character
    s.push('s');
    print_str(&s);

    // You can concetinate strings, but this is a bit tricky due to how the add method is implemented
    // It takes ownership over the first part and needs a reference to all other parts
    let s1 = String::from("Hello");
    let s2 = String::from(" people");
    let s = s1 + &s2;
    print_str(&s);

    // A simpler way is to use the format!() macro, which is similar to println!()
    let s1 = String::from("Hello");
    let s2 = String::from(" people");
    let s = format!("{s1}{s2}");
    print_str(&s);

    // When you need iterate over a String you have to be explicit on what you want
    let s = String::from("др");
    // The .char() method gives you the individual characters of a String
    for c in s.chars() {
        println!("{c}");
    }
    // A valid unicode character may be made up of more than one byte
    // The .bytes() method gives you the individual bytes that make up the String
    for b in s.bytes() {
        println!("{b}");
    }

    let s = String::from("Hello people");
    let s = s.replace("people", "humans");
    print_str(&s);

}

// This function accepts a string literal
fn print_str(s: &str) {
    println!("{}", s);
}