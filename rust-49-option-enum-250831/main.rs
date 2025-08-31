enum Shape {
    Rectangle,
    Circle,
}

fn main() {
    // We call the plus_one() function and assing its return value to a variable
    // This case stores Some(21) in the variable since its input holds value
    let num_added = plus_one(Some(20));
    // This one does not contain an input variable, so it returns None
    let num_absent = plus_one(None);

    println!("The numbers are {num_added:?} and {num_absent:?}");


    // You can use the match control flow anywhere
    // But it needs o be exhaustiv, meaning every possible case must be covered
    // You can use the _ to tell that this should be done for all cases that are not explicitly defined
    let dice_roll = 4;
    match dice_roll {
        1 => println!("You did it and got a one."),
        _ => println!("You did not roll a one."),
    }

    let shape1 = Shape::Rectangle;
    tell_shape(shape1);
}

// This function takes an Option<i32> as input and also returns one
// The match checks if Some() is present and adds one the the value it holds
// When it is not presnet it returns None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn tell_shape(s: Shape) {
    match s {
        Shape::Rectangle => println!("It is a rectangle."),
        Shape::Circle => println!("It is a cricle"),
    }
}