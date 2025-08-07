fn main() {
    // Create a immutable variable and initialize it with 20, stored on teh stack
    let x = 20;

    // Create a reference to the value stored in the variable and store it in another variable
    let r = &x;

    // While you can print the value r is pointing to using the println!() macro
    println!("The value of x is: {r}");
    
    // When you try to use that reference (a pointer to the original variable) the compiler gives an error
    // if r == 20 {println!("It is 20")};

    // To get the value the reference is pointing to, you need to derefernce it using the * symbol
    if *r == 20 {println!("Its is 20")};

    println!("The value r is pointing to is: {}", *r);
}