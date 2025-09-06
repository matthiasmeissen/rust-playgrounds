// Module in same file
mod local_module {
    pub fn prepare_module() {
        println!("Module Prepared.");
    }
}

// Import module from file
mod utils;

use utils::show_details;
use utils::graphics::draw_frame;

fn main() {
    local_module::prepare_module();

    show_details();

    draw_frame();

    utils::graphics::frame_details();
}