use std::fmt::{Debug, Display};

fn main() {
    let numbers1 = vec![10, 40, 30, 80, 60];
    println!("The largest number in {:?} is: {}", numbers1, largest(&numbers1));

    let numbers2 = vec![0.2, 0.6, 0.8, 1.2, 1.4, 0.3];
    println!("The largest number in {:?} is: {}", numbers2, largest(&numbers2));

    let characters = vec!['a', 'c', 'd', 'l', 'b', 'e'];
    println!("The largest character in {:?} is: {}", characters, largest(&characters));

    println!("--------");
    print_largest(&numbers1, "number");
    print_largest(&numbers2, "number");
    print_largest(&characters, "character");
}

fn largest <T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn print_largest1 <T: PartialOrd + Display + Debug>(list: &[T]) {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest element from {:?} is: {}", list, largest);
}

fn print_largest <T, U>(list: &[T], name: U) 
where 
    T: PartialOrd + Display + Debug,
    U: Display,
{
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest {} from {:?} is: {}", name, list, largest);
}