fn main() {
    let num = 4;
    let result = is_even(num);

    if result {
        println!("The number {num} is even.");
    } else {
        println!("The number {num} is odd.");
    }

    // An if statement is an expression
    // This means you can use it for example to initialize a variable
    // But the results must have the same type
    let word = if result {"even"} else {"odd"};
    println!("The number {num} is {word}");
}

// Create a function that expects a 32bit integer and returns a boolean
fn is_even(x: i32) -> bool {
    // The condition inside the if statement must result in a boolean
    // Note that you do not the ()
    if x % 2 == 0 {
        // You can leave out the return keyword and semicolon
        // The function will automatically return the last expression it finds
        true
    } else {
        false
    }
}