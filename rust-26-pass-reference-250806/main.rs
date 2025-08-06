
fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store pointer, length and capacity in immutable variable
    let my_string = String::from("Hello");

    let len = calculate_length(&my_string);
}

fn calculate_length(input: &String) -> usize {
    input.len()
}
