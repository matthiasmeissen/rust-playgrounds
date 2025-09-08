#[allow(dead_code)]

fn main() {
    // Create a mutable vec of type <i32> and use a macro initialize it with values
    let mut v = vec![1, 4, 8, 17, 200];

    // Print the whole vec with the debug format specifier
    // The vec<T> type is an enum that has implemented the debug trait already
    println!("v is {:?}", v);

    
    // vec_name[index] is a shorthand for *vec_name.index(index) 
    // The .index() method returns a reference to the value at index (&i32)
    // The * dereferences it and since i32 has implemented the Copy trait you can assign it to a new variable
    // This means that second now hols a completely new value that is not connected to the vec anymore

    // &vec_name[index] is creating a reference to the value at index in the vector
    // Meaning you create an &i32 that points to the value on the heap
    
    // You can access an element with the vec_name[index] or &vec_name[index] syntax
    // But this will crash the program when index is out of bounds
    let second_a = v[1];
    let second_b = &v[1];
    println!("The copied second value of v is: {second_a}");
    println!("The referenced second value of v is: {second_b}");

    // A safer way to access a value from a vec<T> is to use the .get() method
    // It returns an Option<T> which requires to specify what should happen index it out of bounds 
    // In this case it returns None
    let third = v.get(2);
    match third {
        Some(i) => println!("The third value of v is: {i}"),
        None => println!("Index out of bounds."),
    }

    // To iterate over an array you can use a for loop
    // In this case i is a pointer the the value in the vec
    // And &vec_name is a pointer to the vec itself
    // This is an immutable borrow, meaning you can only read the values
    for i in &v {
        println!("{i}");
    }

    // To be able to modify the values inside the vec
    // You need to set the vec itself as mutable
    // And then create a mutable reference to it
    for i in &mut v {
        // i is now a mutable reference to each element
        // In order to change it you must use * to dereference the value
        *i *= 2;
        println!("Doubled: {i}");
    }

}