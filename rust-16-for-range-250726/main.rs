
fn main() {
    // Iterate through rang 1-3 (last number is excluded)
    for num in 1..4 {
        // Print current index
        println!("{num}");
    }

    // Iterate through numbers 1-3 but reversed using the rev methon on the range type
    for num in (1..4).rev() {
        println!("{num}")
    }
}
