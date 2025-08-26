
#[derive (debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 200,
        height: 120,
    };

    println!("rect1 is {:?}", rect1);
}
