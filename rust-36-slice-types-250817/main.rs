
fn main() {
    let nums = [4, 8, 12, 16, 48, 80];
    let num_slice = &nums[0..3];
    let slice_len = num_slice.len();
    println!("The length of num_slice is: {slice_len}");
}
