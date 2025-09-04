use project::front_of_house::hosting::*;
use project::front_of_house::serving::*;

fn main() {
    seat_guest();
    take_order();
    serve_order();

    serve_and_take_order();
}