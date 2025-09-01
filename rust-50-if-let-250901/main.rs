
enum Shape {
    Rectangle,
    Circle,
}

fn main() {
    let shape1 = Shape::Rectangle;
    get_shape_details(shape1);
}

fn get_shape_details(s: Shape) {
    match s {
        Shape::Rectangle => println!("It is a rectangle."),
        Shape::Circle => println!("It is a circle"),
    }
}
