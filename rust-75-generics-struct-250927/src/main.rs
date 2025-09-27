
#[derive(Debug)]
struct StaticPoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let static_i32_point = StaticPoint{x: 4, y: 8};

    let integer_point = Point{x: 4, y: 8};
    let float_point = Point{x: 0.2, y: 0.8};

    println!("{:?}", static_i32_point);
    println!("{:?}", integer_point);
    println!("{:?}", float_point);
}
