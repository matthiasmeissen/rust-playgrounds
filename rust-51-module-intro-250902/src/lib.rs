
// To organize your code there is a thing called modules
// It is similar to a filesystem, where you place related items in the same folder
// You can also nest things to create a hierarchy when needed
// All modules and things inside are private by default (they can olny be used inside the modles themselves)
// To make modules and elements inside you need to add the pub keyword to them

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist.");
        }

        pub fn seat_at_table(num: u32) {
            println!("Seated {num} guests at table.");
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
