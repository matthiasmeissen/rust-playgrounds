
fn main() {
    let nums = vec![1, 2, 3, 4];

    // Create iterator from nums
    let mut nums_iter = nums.iter();

    // An iterator trait must provide one method called .next()
    // To use that the variable that holds the iterator must be mutable, since we store the current index with it

    let first_option = nums_iter.next();
    // The .next() method does not return a value directly, but instead an option
    // This is for good reason, since there might be a value at that point or ther might be none

    match first_option {
        Some(n) => println!("First item in nums is {n}"),
        None => println!("Out of bounds"),
    }
}
