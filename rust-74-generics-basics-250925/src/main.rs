
fn main() {
    let numbers = vec![80, 40, 20, 62, 140];

    let mut largest = numbers[0];

    for num in numbers {
        if num > largest {
            largest = num;
        }
    }

    println!("The largest number is {largest}");
}
