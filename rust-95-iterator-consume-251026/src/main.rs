fn main() {
    let nums = vec![1, 2, 3, 4];

    // Create iterator from nums
    let mut nums_iter = nums.iter();

    // An iterator trait must provide one method called .next()
    // To use that the variable that holds the iterator must be mutable, since we store the current index with it

    let first_option = nums_iter.next();
    // The .next() method does not return a value directly, but instead an option
    // This is for good reason, since there might be a value at that point or ther might be none
    // To define what should happen on either of those options we use a match statement
    match first_option {
        Some(n) => println!("First item in nums is {n}"),
        None => println!("Out of bounds"),
    }

    // The powerfull thing about iterators is that we can do some compact calculations with them
    // With the .sum() method you can calculate the sum of all elements in an iterator
    let sum: i32 = nums_iter.sum();
    println!("The sum of nums_iter is: {sum}");
    // Note that this sum somehow ignores the 1
    // This is because we have already used it with the .next() method

    // To get the correct sum of nums we need to create a new iterator from that
    let nums_iter2 = nums.iter();
    let sum1: i32 = nums_iter2.sum();
    println!("The sum of nums_iter2 is: {sum1}");

    // The following line is not valid, since takes ownership over the variable
    //println!("{:?}", nums_iter2);

    // There are other usefill things we can do with iterators
    // The .map() method takes a closure and performs an action on all items on the iterator
    let nums_iter3 = nums.iter().map(|x| x * 2);
    // The .map() method takes the iterator and returns a new iterator
    // We could use a for loop, which takes an iterator and performs an action over each item in it
    for num in nums_iter3 {
        println!("{num}");
    }

    // But we can also use a .collect() method to bring it back into a collection like Vec<T>
    // For that we need to create a new iteartor, since the for loop has taken ownership
    let nums_iter4 = nums.iter().map(|x| x * 2);
    // In this case the map on nums is not performed (since iterators are lazy) until we call .collect() in it
    // Note that we need to annotate the type explicitly here
    let new_nums: Vec<_> = nums_iter4.collect();
    println!("The new values are: {:?}", new_nums);


    // We could chain them for more complex things
    let nums_iter5: Vec<_> = nums.iter().map(|x| x + 5).filter(|x| x % 2 == 0).collect();
    println!("Add 5 to every item and only use even ones: {:?}", nums_iter5);
}