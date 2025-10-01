
fn main() {
    println!("Hello, world!");

    let numbers1 = vec![10, 40, 30, 80, 60];
    println!("The largest number in {:?} is: {}", numbers1, largest(&numbers1));

}

fn largest <T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
