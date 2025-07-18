
fn main() {
    // The array type must have a fixed length and all elements inside the same type
    let arr = [0.2, 4000.38, 235.89026893];
    println!("First element of arr is: {0}", arr[0]);

    // Arrays are usefull, when you know the amount of elements will not change
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("The second day of the week is: {0}", days[1]);

    // You can specifiy the type along with the number of elements with let name: [type; num] = [elements]
    let numbers: [f32; 5] = [0.1, 0.2, 0.8, 1.4, 2.6];
    println!("First index is: {0} and last index is: {1}", numbers[0], numbers[4]);
}
