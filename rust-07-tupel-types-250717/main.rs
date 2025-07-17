
fn main() {
    // A tuple holds a collection of values 
    // The type and amount of values is not allowed to change
    let tup = (400, 3.2, 1);

    // You can use a value from the collection by writing tupelName.index
    println!("The first value of tup is {0} the second is {1}", tup.0, tup.1);

    // You can destructure a tuple into seperate variables again with let (tuplen elements) = tupelName
    let (x, y, z) = tup;
    println!("The values for tup as variables are: {x}, {y}, {z}");

    // You can also specify types explicitly for each element
    let tup2: (u32, f32, char) = (80_000, 35253.27, 'R');
    println!("The binary value of {0} is {1:b}",tup2.2, tup2.2 as u32);
}
