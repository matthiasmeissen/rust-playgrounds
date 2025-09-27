// When you define a struct you need to add the type to each member
#[derive(Debug)]
struct StaticPoint {
    x: i32,
    y: i32,
}

// For generic structs (where the type can be anything)
// We need to replace the type with T and the add <T> after the name
// This indicates that the type for each member can be anything, but all the same
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// To allow different types for each member 
// you would add <T, U> or something similar to it
#[derive(Debug)]
struct FlexiblePoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let static_i32_point = StaticPoint{x: 4, y: 8};

    let integer_point = Point{x: 4, y: 8};
    let float_point = Point{x: 0.2, y: 0.8};

    let both_point = FlexiblePoint{x: 4, y: 0.2};

    println!("{:?}", static_i32_point);
    println!("{:?}", integer_point);
    println!("{:?}", float_point);
    println!("{:?}", both_point);
}