#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

fn main() {
    // A vector is a growable collection of things
    let mut num = Vec::new();
    // You can add elements by using the push() method on an instance of a vec struct 
    num.push(20);
    num.push(40);
    println!("Num is {:?}", num);

    // You can use the vec![] macro to create a new vector and initialize it with values
    let v = vec![20, 40, 80, 120, 200];
    println!("The values in v are: {:?}", v);

    // You can get a reference to a specific index in a vector with &vec_name(index)
    // When the values is out of bounds the program will crash
    let second = &v[1];
    println!("The second value in v is {second}");

    // You can also get a value out of a vector by using the get() method
    // This returns an Option<T> type so in order to access the value you must use the match control flow
    // This ensures that you explicitly handle the case when no value is present
    let third = v.get(2);
    match third {
        Some(third) => println!("The third value of v is {third}"),
        None => println!("This index was out of bounds."),
    };

    // You can specify the type when you want
    // When you not do it the compiler automatically infers the type by its first usage
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(20, 20));
    points.push(Point::new(80, 80));
    println!("Points is: {:?}", points);

}