use std::fmt::Debug;

// You can write functions that can accept any type
// fn some_name<T>(param: T) -> T {}
// This means, this function can take in any type and used this type as parameter and return value
// However this does not work for most things you do in functions, since some things can only be done with specific types


fn main() {
    let numbers = vec![80, 40, 20, 62, 140];

    get_largest_number(&numbers);

    let chars = vec!['a', 'd', 'k', 'c', 's'];

    let largest_char = largest(&chars);
    println!("The largest char is {}", largest_char);

    print_any_type("Hello");
}

// What is the difference of passing &[i32] and Vec<i32>
fn get_largest_number(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];

    for num in numbers {
        if num > largest {
            largest = num;
        }
    }

    println!("The largest number in {:?} is {}", numbers, largest);

    largest
}

// This function expect any type and wants to print it with the debug format specifier
// Since not every type has this trait, we need to annotate it by writing <T: Debug>
fn print_any_type<T: Debug>(value: T) {
    println!("The value is: {:?}", value);
}

// Similar this functions does comparison between two instances of a type
// For this to work, we need to write <T: PartialOrd>
// This helps to make thinks work inside the functions
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}