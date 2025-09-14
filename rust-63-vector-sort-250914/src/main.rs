
fn main() {
    let numbers = [20, 40, 80, 42, 68, 200, 30, 62, 120, 61];

    let mut sum = 0;
    for value in numbers {
        sum += value;
    }
    let median = sum / numbers.len();
    println!("The median is: {median}");
}
