
pub fn cook_order() {
    println!("Order Cooked from seperate file.");
}

pub mod prepare {
    pub fn prepare_food() {
        println!("Food prepared from module in seperate file.");
    }
}
