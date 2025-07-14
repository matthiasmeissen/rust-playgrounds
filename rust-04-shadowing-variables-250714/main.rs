// Shadowing lets us redeclare a variable with the same name and even change its type
// This is for example helpful if we want to convert some input to a number without coming up with a new name for it

fn main() {
    // Shadowing 1
    // Declare a variable and initialize to string
    // Then redeclare with the same name and initialize with int 
    // This means we have shadowd the initial variable
    let space = " ";
    println!("The value of space is: {space}");
    let space = 1;
    println!("The value of space is: {space}");


    println!("--------");

    // Shadowing 2
    // Declare a variable and initialize to string
    // Then redeclare with the same name and use a function on the initial value, which converts it to int
    let spaces = "  ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    println!("--------");

    // Shadowing 3
    // Use the variable and redeclare it again, but inside another scope
    // Use print to see the changes
    // Print again after the scope to see that the initial variable is preserved
    {
        let spaces = spaces * 4;
        println!("The value of spaces in the inner scope is: {spaces}");
    }
    println!("The value of spaces after the scope is: {spaces}");
}