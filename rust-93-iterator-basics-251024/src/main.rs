fn main() {
    let mut nums = vec![1, 2, 3, 4];

    print_direct(&nums);

    print_iterator(&nums);

    print_iterator_next(&nums);

    iterate_and_multiply(&mut nums);
}

fn print_direct(nums: &[i32]) {
    // Takes in a slice reference to a collection of elements of type i32
    // This line iterates over the elements and prints them
    // In this case the compiler automatically calls the .into_iter() methods on the input
    // This creates an immutable reference to each element (&i32)
    for num in nums {
        println!("{num}");
    }
}

fn print_iterator(nums: &[i32]) {
    //Converts the input to iterator
    let nums_iter = nums.iter();

    // This also iterates over the items and prints them 
    // but the have been converted into an iterator, whatever the difference is
    for num in nums_iter {
        println!("{num}");
    }
}

fn print_iterator_next(nums: &[i32]) {
    // The iterator trait requires only one methode to be defined .next()
    // The .next() method returns on Option<T> 
    // Which if there is another item in the sequence returns Some(item) or if not returns None
    // To do that you need to declare variable holding the iterator as mutable, since the state changes with each call
    let mut nums_iter = nums.iter();
    for i in 0..10 {
        match nums_iter.next() {
            Some(num) => println!("Item {i}: {num}"),
            None => println!("No more items to consume"),
        }
    }
}

fn iterate_and_multiply(nums: &mut[i32]) {
    let nums_iter = nums.iter_mut();
    for num in nums_iter {
        *num *= 2;
    }

    println!("{:?}", nums);
}