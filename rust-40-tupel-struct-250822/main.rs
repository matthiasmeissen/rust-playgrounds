
struct Point (u32, u32);

fn main() {
    let point1 = Point(0, 4);
    println!("Point x is {point1.0} and point y is {point1.1}");
}
