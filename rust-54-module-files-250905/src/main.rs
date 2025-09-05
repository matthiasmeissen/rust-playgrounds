// Every project has a crate
// The module system within a crate froms a tree structure
// The root is typically src/main.rs (which is for creating binaries, when you build a libarary this is src/lib.rs)
// From there the compiler builds the module tree

// mod
// The mod keyword means that a module exists
// Either after the {} or as a seperate file with the same name

// LOCAL MODULES
// This module exists after the {}
mod greeter {
    // To access a function you need to make it public (ist is private on default)
    pub fn greet() {
        println!("Hello people");
    }
}

// This creates a namespace that lets use use a module with a shorter path
// You could be even more specific and write crate::greeter::greet which is the absolute path
// But main.rs is equal to crate so we are already in it
use greeter::greet;

// EXTERNAL MODULES - DIRECT FILE
// This module exists in a file called cooking.rs
mod cooking;
// Now we can call the functions and modules inside it
use cooking::prepare::prepare_food;

// EXTERNAL MODULES - FOLDERS

mod garden;


fn main() {
    // LOCAL MODULES
    // To use the function from greeter module in main, we need to write its path
    greeter::greet();
    // Since we have written use greeter::greet, we can now call the function without path here
    greet();

    // EXTERNAL FILE - DIRECT FILE
    cooking::cook_order();
    prepare_food();

    // EXTERNAL FILE - FOLDERS
    garden::create_garden();
    garden::buildings::create_building();
    garden::plants::create_plant();

}