
#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 400,
        height: 200,
    };

    println!("rect1 is: {}", rect1);
}
