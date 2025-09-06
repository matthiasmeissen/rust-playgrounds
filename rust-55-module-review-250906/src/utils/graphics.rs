
pub fn draw_frame() {
    println!(" --------- ");
    println!("|         |");
    println!("|         |");
    println!(" --------- ")
}

pub fn frame_details() {
    println!("Called from submodule.");
    // Refer to function in parent module
    super::show_details();
}
