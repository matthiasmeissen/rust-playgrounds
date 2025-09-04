pub mod front_of_house {
    pub mod hosting {
        pub fn seat_guest() {
            println!("Guest seated.");
        }
        pub fn take_order() {
            println!("Order taken.");
        }
    }
    pub mod serving {
        pub fn serve_order() {
            println!("Order served.");
            super::hosting::take_order();
        }
        pub fn serve_and_take_order() {
            println!("First order served, asking to take new:");
            // super allows you to specify a path from the parent module
            // In this case the function we are in right now is in the serving module
            // Using super we go into its parent which is front of house
            // From there we can go into the hosting module and call the take order function
            super::hosting::take_order();
        }
    }
}