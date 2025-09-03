// This line tells the compile to ignore unused functions and variables
#[allow(dead_code)]

// You can define modules inside the main file as well
// This is not very usefull for modularisation, but helps in this case
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist.");
        }

        fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {
            println!("Order taken.");
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

// You can shorten the path to a module with the use keyword
use crate::front_of_house::hosting;

use crate::front_of_house::serving::take_order;

fn main() {
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    take_order();
}
