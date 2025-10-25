fn main() {
    let mut nums = vec![1, 2, 3, 4];

    simple_iter(&nums);

    explicit_iter(&nums);

    // This changes nums from [1, 2, 3, 4] to [3, 4, 5, 6]
    modifying_iter(&mut nums);

    println!("{:?}", nums);

    get_sum(&nums);

    multiply_elements(&nums);
}

fn simple_iter<T: std::fmt::Display>(nums: &[T]) {
    for num in nums {
        println!("{num}");
    }
}

fn explicit_iter<T: std::fmt::Display>(nums: &[T]) {
    let nums_iter = nums.iter();

    for num in nums_iter {
        println!("{num}");
    }
}

fn modifying_iter(nums: &mut[i32]) {
    let mut nums_iter = nums.iter_mut();

    for num in nums_iter {
        *num += 2;
    }
} 

fn get_sum(nums: &[i32]) {
    let nums_iter = nums.iter();
    let sum: i32 = nums_iter.sum();
    println!("{sum}");
}

fn multiply_elements(nums: &[i32]) {
    let mult: Vec<_> = nums.iter().map(|x| x * 2).collect();
    println!("{:?}", mult);
}