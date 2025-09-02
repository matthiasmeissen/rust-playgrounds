// To use modules and associated elements in them you need import them
// This is done with the use keyword followed by the path to the module
// Not that the first element is always the name of your library, which can be set in the Cargo.toml file
// From there you can specify what you need
use project::front_of_house::hosting;
use project::front_of_house::serving;

fn main() {
    // To use a function from a module you can write :: and then the name of the function
    hosting::add_to_waitlist();
    hosting::seat_at_table(2);

    serving::take_order();
}