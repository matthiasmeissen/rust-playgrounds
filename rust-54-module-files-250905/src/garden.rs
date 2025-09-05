
pub mod plants;

pub fn create_garden() {
    println!("Garden Created - From seperate file.");
}

pub mod buildings {
    pub fn create_building() {
        println!("Building Created - From Submodule in File")
    }
}
